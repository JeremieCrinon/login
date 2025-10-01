import Vapor
import JWT

struct TokenPayload: JWTPayload {
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
