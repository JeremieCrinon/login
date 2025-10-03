@testable import App
import VaporTesting
import Testing
import Fluent

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

func createTestUser(on db: any Database, roles: [Role]) async throws -> User {
    let password = "Admin12345@"
    let password_hash = try Bcrypt.hash(password)
    let user = User(email: "test@mail.com", password: password_hash, roles: roles)
    try await user.save(on: db)
    return user
}
