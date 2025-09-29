import Vapor
import Mailgun


struct UsersController: RouteCollection {

    func boot(routes: any RoutesBuilder) throws {
        let users = routes.grouped("users").grouped(AuthMiddleware(requiredRole: .new_account))
        
        users.post("new", ":lang", use: create)
    }

    struct CreateUserRequest: Content, Validatable {
        let email: String
        let roles: [Role]

        static func validations(_ validations: inout Validations) {
            validations.add("email", as: String.self, is: .email)
        }
    }

    // This is the controller for the /users/new/:lang route, it creates a new user in DB with a temporary passord, and send an invite email to the new user with the temporary password
    func create(req: Request) async throws -> HTTPStatus {

        try CreateUserRequest.validate(content: req) // Validate that the request matches the CreateUserRequest, which means that it is valid
        let input = try req.content.decode(CreateUserRequest.self)
        
        // We return a transaction, that will either return an ok status, or an error (if case of an error, everything done in DB in the transaction will be undone)
        return try await req.db.transaction { database in
            // We make a request in DB for a user with the same email that has been sent
            let existing_user = try await User.query(on: database)
                .filter(\.$email, .equal, input.email)
                .first()

            if existing_user != nil { // If we found a user in DB with the same email that has been sent, we return an error
                throw Abort(.conflict, reason: "Email is already used by another account.")
            }

            let password = generatePassword(length: 12) // Call the generatePassword helper function to generate a random password to the new user
            let password_hash = try req.password.hash(password) // Hash the password for storing in DB

            var roles = input.roles
            if !roles.contains(.new_account) { // If the roles array sent by the user does not contain the new account role
                roles.append(.new_account) // Add the new account role to the roles array
            }

            let new_user = User(email: input.email, password: password_hash, roles: roles) // Create a new user
            try await new_user.save(on: database) // Save in DB

            // Send invitation email with the password
            //TODO: Make a clean email with internationalization

            let message = MailgunMessage(
                from: Environment.get("MAILGUN_EMAIL") ?? "email@example.com",
                to: new_user.email,
                subject: "Create you new account",
                text: "Here is your password : \(password)",
            )

            do {
                let _ = try await req.mailgun().send(message).get()
            } catch {
                req.logger.error("An error occured sending the email via mailgun : \(error.localizedDescription)")
                throw Abort(.internalServerError)
            }
            
            return .ok
        }
    }
}
