import Vapor
import JWT

struct LoginController: RouteCollection {

    func boot(routes: any RoutesBuilder) throws {
        let no_role = routes.grouped(AuthMiddleware(requiredRole: nil)) // The group for routes that requires a logged in user and any role
        let new_account = routes.grouped(AuthMiddleware(requiredRole: .new_account)) // The groupe for the new accounts, used only by the modify_new_account route

        no_role.get("user-infos", use: userInfos)
        new_account.post("modify-new-account", use: modifyNewAccount)

        routes.post("login", use: login)
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

}
