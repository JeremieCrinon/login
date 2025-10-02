import Fluent
import Vapor

struct CreateUser: AsyncMigration {
    func prepare(on database: any Database) async throws {
        try await database.schema("users")
            .id()
            .field("email", .string, .required)
            .field("password", .string, .required)
            .field("email_verification_code", .string)
            .field("password_reset_code", .string)
            .field("roles", .array(of: .string), .required)
            .field("created_at", .datetime, .required)
            .field("updated_at", .datetime, .required)
            .create()

        let password = "Admin12345@" // First user default password
        let password_hash = try Bcrypt.hash(password) // Hash the password to put it in database

        // Prepare the first user
        let firstUser = User(
            email: "email@mail.com",
            password: password_hash,
            roles: [.admin, .new_account] // Give to the user the admin role so it can add other users, and do what they want, and the new_account role so they will be prompted to change their email and password on first login
        )

        try await firstUser.save(on: database)
    }

    func revert(on database: any Database) async throws {
        try await database.schema("users").delete()
    }
}

