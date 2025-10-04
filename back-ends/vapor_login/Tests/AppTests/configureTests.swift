@testable import App
import VaporTesting
import Testing
import Fluent
import JWT

func withAppIncludingDB(_ test: (Application) async throws -> ()) async throws {
    let app = try await Application.make(.testing)
    do {
        try await configure(app)
        try await app.autoMigrate()
        try await test(app)
        try await app.autoRevert()
    } catch {
        try? await app.autoRevert()
        try await app.asyncShutdown()
        throw error
    }
    try await app.asyncShutdown()
}

// This function creates a user in DB with the roles it has been given. This is usefull for tests that requires a user.
func createTestUser(on db: any Database, roles: [Role]) async throws -> User {
    let password = "Admin12345@"
    let password_hash = try Bcrypt.hash(password)
    let user = User(email: "test@mail.com", password: password_hash, roles: roles)
    try await user.save(on: db)
    return user
}

// This function creates a JWT for a user
func getJWT(user: User, app: Application) async throws -> String {
    let tokenExpirationTime = 3600
    let expirationDate = Date().addingTimeInterval(TimeInterval(tokenExpirationTime))
    let expiration = ExpirationClaim(value: Date(timeIntervalSince1970: floor(expirationDate.timeIntervalSince1970)))

    let payload = TokenPayload(
        subject: SubjectClaim("authorization"),
        expiration: expiration,
        userId: try user.requireID()
    )

    let token = try await app.jwt.keys.sign(payload)


    return token
}

// This function creates a user with the given roles, and returns a JWT for the new user
func createTestUserAndGetJWT(app: Application, roles: [Role]) async throws -> String {
    let user = try await createTestUser(on: app.db, roles: roles)

    return try await getJWT(user: user, app: app)
}
