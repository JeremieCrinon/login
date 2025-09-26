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

    struct tokenPayload: JWTPayload {
        enum CodingKeys: String, CodingKey {
            case subject = "sub"
            case expiration = "exp"
            case userId = "user"
        }

        var subject: SubjectClaim

        var expiration: ExpirationClaim

        var userId: UUID

        func verify(using algorithm: some JWTAlgorithm) async throws {
            try self.expiration.verifyNotExpired()
        }
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

        let payload = tokenPayload(
            subject: SubjectClaim("authorization"),
            expiration: .init(value: Date().addingTimeInterval(TimeInterval(tokenExpirationTime))),
            userId: try user!.requireID()
        )

        let token = try await req.jwt.sign(payload)

        return LoginResponse(token: token)

    }

}
