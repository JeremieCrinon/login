import Vapor
import JWT
import Mailgun

struct LoginController: RouteCollection {

    func boot(routes: any RoutesBuilder) throws {
        let no_role = routes.grouped(AuthMiddleware(requiredRole: nil)) // The group for routes that requires a logged in user and any role
        let new_account = routes.grouped(AuthMiddleware(requiredRole: .new_account)) // The groupe for the new accounts, used only by the modify_new_account route
        let unverified_email = routes.grouped(AuthMiddleware(requiredRole: .unverified_email))
        let logged_in = routes.grouped(AuthMiddleware(requiredRole: .user)) // The route for the logged in users that requires them to not have a new_account or unverified_email

        no_role.get("user-infos", use: userInfos)
        no_role.post("edit-email", ":lang", use: changeEmail)

        new_account.post("modify-new-account", ":lang", use: modifyNewAccount)

        unverified_email.post("verify-email", use: verifyEmail)

        logged_in.post("edit-password", use: changePassword)

        routes.post("login", use: login)
        routes.post("forgot-password", ":lang", use: forgotPassword)
        routes.post("reset-password", use: resetPassword)
    }

    struct UserInfosResponse: Content {
        let roles: [Role]
        let user_mail: String
    }

    func userInfos(req: Request) async throws -> UserInfosResponse {
        let user = req.user!

        let response = UserInfosResponse(
            roles: user.roles,
            user_mail: user.email
        )

        return response
    }

    struct LoginRequest: Content {
        let email: String
        let password: String
    }

    struct LoginResponse: Content {
        let token: String
    }

    func login(req: Request) async throws -> LoginResponse {

        let input = try req.content.decode(LoginRequest.self)

        // Get the user to verify it has the same password has the one sent
        let user = try await User.query(on: req.db)
            .filter(\.$email, .equal, input.email)
            .first()

        if user == nil { // If we haven't found a user with the email...
            throw Abort(.badRequest, reason: "Email or password incorrect") // We return the same error as if the password isn't correct, so the client cannot know if an email adress is registered on the app
        }

        // Verify that the password sent is valid
        if try await !req.password.async.verify(input.password, created: user!.password) {
            throw Abort(.badRequest, reason: "Email or password incorrect")
        }

        let tokenExpirationTime = Int(Environment.get("JWT_AUTHORIZATION_EXPIRATION_TIME") ?? "3600") ?? 3600
        let expirationDate = Date().addingTimeInterval(TimeInterval(tokenExpirationTime))
        let expiration = ExpirationClaim(value: Date(timeIntervalSince1970: floor(expirationDate.timeIntervalSince1970)))

        // Prepare the payload for the JWT
        let payload = TokenPayload(
            subject: SubjectClaim("authorization"),
            expiration: expiration,
            userId: try user!.requireID()
        )



        let token = try await req.jwt.sign(payload) // Sign the JWT

        return LoginResponse(token: token)

    }

    struct ModifyNewAccountRequest: Content, Validatable {
        let new_email: String
        let new_password: String

        static func validations(_ validations: inout Validations) {
            validations.add("new_email", as: String.self, is: .email)
            validations.add("new_password", as: String.self, is: .password)
        }
    }

    func modifyNewAccount(req: Request) async throws -> HTTPStatus {
        try ModifyNewAccountRequest.validate(content: req)
        let input = try req.content.decode(ModifyNewAccountRequest.self)
        let user = req.user!

        // Make a transaction so any DB interaction made here will be undone if there is an error
        return try await req.db.transaction { database in
            // If a user with the new email that is not the user that called this route (the user that called this route might not want to change their email) exists, we return a 409 code
            let email_conflict_user = try await User.query(on: database)
                .filter(\.$email, .equal, input.new_email)
                .filter(\.$id, .notEqual, user.id!)
                .first()

            if email_conflict_user != nil {
                throw Abort(.conflict, reason: "Email is already used by another account.")
            }

            // Hash the password from request
            let password_hash = try req.password.hash(input.new_password)

            // Remove the new_account role for the user roles
            var roles = user.roles
            
            if let roleIndex = roles.firstIndex(of: .new_account) {
                roles.remove(at: roleIndex)
            }

            // Update the user in DB with the new email, password, and roles
            try await User.query(on: database)
                .set(\.$email, to: input.new_email)
                .set(\.$password, to: password_hash)
                .set(\.$roles, to: roles)
                .filter(\.$id, .equal, user.id!)
                .update()

            // Fetch the updated user
            let user = try await User.query(on: database)
                .filter(\.$id, .equal, user.id!)
                .first()!

            // Call the helper function to put the verify_email role to the user and send them an email with a code
            try await sendEmailVerificationToUser(user: user, req: req, db: database)

            return .ok
        }

    }

    struct verifyEmailRequest: Content {
        let code: String
    }

    func verifyEmail(req: Request) async throws -> HTTPStatus {
        let input = try req.content.decode(verifyEmailRequest.self)

        let user = req.user!

        // Verify the code sent is the user's one
        if input.code != user.emailVerificationCode {
            throw Abort(.badRequest, reason: "The code you sent isn't the right one.")
        }

        // Remove the unverified_email role from the roles list
        var roles = user.roles
        if let roleIndex = roles.firstIndex(of: .unverified_email) {
            roles.remove(at: roleIndex)
        }

        // Update the user with the new roles list and remove the email verification code
        try await User.query(on: req.db)
            .set(\.$emailVerificationCode, to: nil)
            .set(\.$roles, to: roles)
            .filter(\.$id, .equal, user.id!)
            .update()

        return .ok
    }

    struct forgotPasswordRequest: Content {
        let email: String
    }

    func forgotPassword(req: Request) async throws -> HTTPStatus {
        let input = try req.content.decode(forgotPasswordRequest.self)

        return try await req.db.transaction { database in

            // Get the user with the send email
            let user = try await User.query(on: database)
                .filter(\.$email, .equal, input.email)
                .first()

            // If we didn't find a user, we don't tell the client
            if user != nil {
                // Generate a reset password code
                let code = generatePassword(length: 12)

                let front_end_url = Environment.get("FRONT_END_URL") ?? ""

                let link: String = "\(front_end_url)/forgot-password/\(code)"
                
                // Update the user with a password reset code
                try await User.query(on: database)
                    .set(\.$passwordResetCode, to: code)
                    .filter(\.$id, .equal, user!.id!)
                    .update()

                // Send the reset password to the user by email
                let message = MailgunTemplateMessage(
                    from: Environment.get("MAILGUN_EMAIL") ?? "email@example.com",
                    to: user!.email,
                    subject: "Reset your password",
                    template: "forgot-password",
                    templateData: ["logo_url": Environment.get("LOGO_URL") ?? "", "link": link]
                )

                if req.application.environment != .testing {
                    let _ = try await req.mailgun().send(message).get()
                }
                
            }

            return .ok
        }

    }

    struct resetPassowrdRequest: Content, Validatable {
        let code: String
        let new_password: String

        static func validations(_ validations: inout Validations) {
            validations.add("new_password", as: String.self, is: .password)
        }
    }

    func resetPassword(req: Request) async throws -> HTTPStatus {
        try resetPassowrdRequest.validate(content: req)
        let input = try req.content.decode(resetPassowrdRequest.self)

        // Get the user corresponding to the code if one exists
        let user = try await User.query(on: req.db)
            .filter(\.$passwordResetCode, .equal, input.code)
            .first()

        // Throw an error if no user has the code sent
        if user == nil {
            throw Abort(.unauthorized, reason: "The code you sent isn't correct")
        }

        // Hash the new password
        let password_hash = try req.password.hash(input.new_password)

        // Modify the user with the new password and remove the password reset code
        try await User.query(on: req.db)
            .set(\.$passwordResetCode, to: nil)
            .set(\.$password, to: password_hash)
            .filter(\.$id, .equal, user!.id!)
            .update()

        return .ok
    }

    struct changeEmailRequest: Content, Validatable {
        let new_email: String
        let password: String?

        static func validations(_ validations: inout Validations) {
            validations.add("new_email", as: String.self, is: .email)
        }
    }

    func changeEmail(req: Request) async throws -> HTTPStatus {
        try changeEmailRequest.validate(content: req)
        let input = try req.content.decode(changeEmailRequest.self)
        let user = req.user!
        
        return try await req.db.transaction { database in
            // If the user calls this route while they don't have a verified email, we do not ask for password for convenience
            if !user.roles.contains(.unverified_email) {
                if input.password == nil {
                    throw Abort(.badRequest, reason: "You need to send your current password as you don't have an account with an unverified email")
                }

                // Verify that the password sent is valid
                if try await !req.password.async.verify(input.password!, created: user.password) {
                    throw Abort(.unauthorized, reason: "The password you sent isn't correct")
                }
            }
                    
            // Check if another user has the new email (the user might send the same email they have already just for having a new email verfication email, so we take that into account)
            let existing_user = try await User.query(on: database)
                .filter(\.$email, .equal, input.new_email)
                .filter(\.$id, .notEqual, user.id!)
                .first()

            if existing_user != nil {
                throw Abort(.conflict, reason: "The new email is already used by another account.")
            }

            // Edit the user now
            try await User.query(on: database)
                .set(\.$email, to: input.new_email)
                .filter(\.$id, .equal, user.id!)
                .update()

            // Fetch the updated user
            let user = try await User.query(on: database)
                .filter(\.$id, .equal, user.id!)
                .first()!

            // Call the helper function to add the verify_email role to the user if they don't have it already, put them an email verification code and sending them an email with it
            try await sendEmailVerificationToUser(user: user, req: req, db: database)

            return .ok

        }

    }

    struct changePasswordRequest: Content, Validatable {
        let current_password: String
        let new_password: String

        static func validations(_ validations: inout Validations) {
            validations.add("new_password", as: String.self, is: .password)
        }
    }

    func changePassword(req: Request) async throws -> HTTPStatus {
        try changePasswordRequest.validate(content: req)
        let input = try req.content.decode(changePasswordRequest.self)
        let user = req.user!

        if try await !req.password.async.verify(input.current_password, created: user.password) {
            throw Abort(.unauthorized, reason: "The password you sent isn't valid")
        }

        let password_hash = try req.password.hash(input.new_password)

        try await User.query(on: req.db)
            .set(\.$password, to: password_hash)
            .filter(\.$id, .equal, user.id!)
            .update()

        return .ok
    }

}
