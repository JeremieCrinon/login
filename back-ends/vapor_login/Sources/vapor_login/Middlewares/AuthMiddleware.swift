import Vapor

struct AuthMiddleware: AsyncMiddleware {
    func respond(to request: Request, chainingTo next: any AsyncResponder) async throws -> Response {
        do {
            let payload = try await request.jwt.verify(as: TokenPayload.self) // Get the content of the JWT in the Authorization header (throws an error if it isn't valid, that will be handled by the catch block)
            print(payload)

        } catch {
            throw Abort(.unauthorized, reason: "Invalid or expired token")
        }

        return try await next.respond(to: request) // If we arrive here, it means we have passed the checks

    }
}
