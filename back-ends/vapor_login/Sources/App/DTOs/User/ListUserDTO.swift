import struct Foundation.UUID
import Vapor

struct ListUserDTO: Content {
    let id: UUID
    let email: String
    let roles: [Role]
    let created_at: Date?
    let updated_at: Date?
}
