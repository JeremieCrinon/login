import Vapor
import Mailgun


struct UsersController: RouteCollection {

    func boot(routes: any RoutesBuilder) throws {
        let users = routes.grouped("users")
        
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
            //TODO: Verify that the email isn't already taken

            let password = generatePassword(length: 12) // Call the generatePassword helper function to generate a random password to the new user
            let password_hash = try req.password.hash(password) // Hash the password for storing in DB

            let new_user = User(email: input.email, password: password_hash, roles: input.roles) // Create a new user
            try await new_user.save(on: database) // Save in DB

            //TODO: Make a clean email with internationalization
            // Send invitation email with the password
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
