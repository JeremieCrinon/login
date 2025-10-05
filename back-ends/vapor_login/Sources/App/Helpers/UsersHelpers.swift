import Vapor
import Crypto
import Foundation
import Fluent
import Mailgun

func generatePassword(length: Int) -> String {
    let characters = Array("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")
    var password = ""

    for _ in 0..<length {
        let i = Int.random(in: 0..<characters.count)
        password.append(characters[i])
    }

    return password
}

// This function add the verify_email role to a user, generates an email verification code and sends them an email with the code
func sendEmailVerificationToUser(user: User, req: Request, db: any Database) async throws -> Void {
    var roles = user.roles

    // Add the unverified_email role to the user
    if !roles.contains(.unverified_email) {
        roles.append(.unverified_email)
    }

    // Generate a verification code
    let code = generatePassword(length: 12)
    
    // Update the user in DB with the email verification code and the new roles
    try await User.query(on: db)
        .set(\.$roles, to: roles)
        .set(\.$emailVerificationCode, to: code)
        .filter(\.$id, .equal, user.id!)
        .update()

    // Send an email with the verification code
    let message = MailgunMessage(
        from: Environment.get("MAILGUN_EMAIL") ?? "email@example.com",
        to: user.email,
        subject: "Verify your email",
        text: "Here is your code : \(code)"
    )

    if req.application.environment != .testing {
        let _ = try await req.mailgun().send(message).get()
    }
}
