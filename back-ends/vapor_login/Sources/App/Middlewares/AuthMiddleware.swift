import Vapor

struct AuthMiddleware: AsyncMiddleware {

    let requiredRole: Role?

    init(requiredRole: Role?) {
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

        // Add the user to the request, so it can be accessed by the controllers if needed
        request.user = user

        // If the required role is nil, it means that the route should be accessible to any user logged in, even users with a new account
        if self.requiredRole == nil {
            return try await next.respond(to: request)
        }

        // If the required role is new_account or unverified_email, we verify that the user has the required role, even if they are admin, these routes must be accessible to user with these roles and only them
        if self.requiredRole == .new_account || self.requiredRole == .unverified_email {
            if !user!.roles.contains(self.requiredRole!) {
                throw Abort(.unauthorized, reason: "This route is only for new accounts or accounts without a verified email adress")
            } else {
                return try await next.respond(to: request)
            }
        }

        // If the user has the new_account role or the unverified email role, they are not allowed here yet (if the route was accessible to them, it will either have a nil required role or the required role new_account or unverified_email)
        if user!.roles.contains(.new_account) || user!.roles.contains(.unverified_email) {
            throw Abort(.unauthorized, reason: "Your account is a new account or has an unverified email, you need to modify your new account or verify your email to continue")
        }

        // If the user has the role admin, they can continue
        if user!.roles.contains(.admin) {
            return try await next.respond(to: request)
        }

        // If we went to here, and the required role is user, it means the user doesn't need any particular role, so we continue
        if self.requiredRole == .user {
            return try await next.respond(to: request)
        }

        // Verify that the user has the required role
        if !user!.roles.contains(self.requiredRole!) {
            throw Abort(.unauthorized, reason: "You don't have the required role to do that")
        }

        // If we arrive here, it means we have passed the checks
        return try await next.respond(to: request) 
    }
}
