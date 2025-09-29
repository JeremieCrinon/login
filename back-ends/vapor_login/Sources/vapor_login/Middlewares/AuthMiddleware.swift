import Vapor

struct AuthMiddleware: AsyncMiddleware {

    let requiredRole: Role

    init(requiredRole: Role) {
        self.requiredRole = requiredRole
    }

    func respond(to request: Request, chainingTo next: any AsyncResponder) async throws -> Response {

        // Get the content of the token in the Authorization header
        guard let payload = try? await request.jwt.verify(as: TokenPayload.self) else {
            throw Abort(.unauthorized, reason: "Invalid or expired token")
        }

        // Get the user corresponding to the token
        let user = try await User.query(on: request.db)
            .filter(\.$id, .equal, payload.userId)
            .first()

        // Check if the user exists in DB
        if user == nil {
            throw Abort(.unauthorized, reason: "User does not exists anymore")
        }

        //TODO: Do the checks for the new_account and unverified_email roles

        // If the user has the role admin, they can continue
        if user!.roles.contains(.admin) {
            return try await next.respond(to: request)
        }

        // If we went to here, and the required role is user, it means the user doesn't need any particular role, so we continue
        if self.requiredRole == .user {
            return try await next.respond(to: request)
        }

        // Verify that the user has the required role
        if !user!.roles.contains(self.requiredRole) {
            throw Abort(.unauthorized, reason: "You don't have the required role to do that")
        }

        return try await next.respond(to: request) // If we arrive here, it means we have passed the checks

    }
}
