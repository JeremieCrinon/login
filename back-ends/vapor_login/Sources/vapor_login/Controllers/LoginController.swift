import Vapor
import JWT

struct LoginController: RouteCollection {

    func boot(routes: any RoutesBuilder) throws {
        routes.post("login", use: login)
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

        if try await !req.password.async.verify(input.password, created: user!.password) {
            throw Abort(.badRequest, reason: "Email or password incorrect")
        }

        let tokenExpirationTime = Int(Environment.get("JWT_AUTHORIZATION_EXPIRATION_TIME") ?? "3600") ?? 3600
        let expirationDate = Date().addingTimeInterval(TimeInterval(tokenExpirationTime))
        let expiration = ExpirationClaim(value: Date(timeIntervalSince1970: floor(expirationDate.timeIntervalSince1970)))

        let payload = TokenPayload(
            subject: SubjectClaim("authorization"),
            expiration: expiration,
            userId: try user!.requireID()
        ) // Prepare the payload for the JWT

        let token = try await req.jwt.sign(payload) // Sign the JWT

        return LoginResponse(token: token)

    }

}
