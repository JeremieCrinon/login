import Vapor
import JWT

struct LoginController: RouteCollection {

    func boot(routes: any RoutesBuilder) throws {
        let no_role = routes.grouped(AuthMiddleware(requiredRole: nil)) // The group for routes that requires a logged in user and any role

        no_role.get("user-infos", use: userInfos)

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

}
