import Foundation
import Fluent
import struct Foundation.UUID
import Vapor

enum Role: String, Codable, Content {
    case Admin
    case User
    case NewAccount
    case UnverifiedEmail
    case EditUsers
}

final class User: Model, @unchecked Sendable {
    static let schema = "users"

    @ID(key: .id)
    var id: UUID?

    @Field(key: "email")
    var email: String

    @Field(key: "password")
    var password: String

    @OptionalField(key: "email_verification_code")
    var emailVerificationCode: String?

    @OptionalField(key: "password_reset_code")
    var passwordResetCode: String?

    @Field(key: "roles")
    var roles: [Role]

    @Timestamp(key: "created_at", on: .create)
    var createdAt: Date?

    @Timestamp(key: "updated_at", on: .update)
    var updatedAt: Date?

    init() { }

    init(
        id: UUID? = nil, 
        email: String, 
        password: String, 
        emailVerificationCode: String? = nil, 
        passwordResetCode: String? = nil, 
        roles: [Role]
    ) {
        self.id = id
        self.email = email
        self.password = password
        self.emailVerificationCode = emailVerificationCode
        self.passwordResetCode = passwordResetCode
        self.roles = roles
    }
}
