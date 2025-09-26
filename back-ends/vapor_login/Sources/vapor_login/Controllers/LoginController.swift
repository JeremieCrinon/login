import Vapor

struct LoginController: RouteCollection {

    func boot(routes: any RoutesBuilder) throws {
        routes.post("login", use: login)
    }

    struct LoginRequest: Content {
        let email: String
        let password: String
    }

    func login(req: Request) async throws -> HTTPStatus {

        let input = try req.content.decode(LoginRequest.self)

        // Get the user to verify it has the same password has the one sent
        let user = try await User.query(on: req.db)
            .filter(\.$email, .equal, input.email)
            .first()

        if user == nil { // If we haven't found a user with the email...
            throw Abort(.badRequest, reason: "Email or password incorrect") // We return the same error as if the password isn't correct, so the client cannot know if an email adress is registered on the app
        }

        if try await !req.password.async.verify(input.password, created: user!.password) {
            throw Abort(.badRequest, reason: "Email or password incorrect")
        }

        //TODO: Generate a JWT and return it

        return .ok

    }

}
