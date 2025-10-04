import Fluent
import Vapor

func routes(_ app: Application) throws {
    app.get { req async in
        "It works!"
    }

    app.get("hello") { req async -> String in
        "Hello, world!"
    }

    // Create simple routes for testing the AuthMiuddleware (avaible just for the unit tests)
    if app.environment == .testing {
        let edit_users = app.grouped(AuthMiddleware(requiredRole: .edit_users))

        edit_users.get("edit_users") { req async -> HTTPStatus in
            return .ok
        }
    }

    try app.register(collection: UsersController())
    try app.register(collection: LoginController())

}
