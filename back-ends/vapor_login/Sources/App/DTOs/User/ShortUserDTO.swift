import struct Foundation.UUID
import Vapor

struct ShortUserDTO: Content {
    let id: UUID
    let email: String
    let roles: [Role]
}
