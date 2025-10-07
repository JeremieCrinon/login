@testable import App
import VaporTesting
import Testing
import JWTKit

@Suite("login routes tests")
struct LoginRoutesTests {
    @Test("Test login route with a wrong email")
    func testLoginWrongEmail() async throws {
        try await withAppIncludingDB { app in
            let _ = try await createTestUser(on: app.db, roles: [.admin])

            let body = LoginController.LoginRequest(email: "test@mail.co", password: "Admin12345@")

            try await app.testing().test(.POST, "login", beforeRequest: { req in
                try req.content.encode(body)
            }, afterResponse: { res async throws in
                #expect(res.status == .badRequest)
            })
        }
    }

    @Test("Test login route with a wrong password")
    func testLoginWrongPassword() async throws {
        try await withAppIncludingDB { app in
            let _ = try await createTestUser(on: app.db, roles: [.admin])

            let body = LoginController.LoginRequest(email: "test@mail.com", password: "Admin12345")

            try await app.testing().test(.POST, "login", beforeRequest: { req in
                try req.content.encode(body)
            }, afterResponse: { res async throws in
                #expect(res.status == .badRequest)
            })
        }
    }

    @Test("Test login route with right credentials")
    func testLoginWithRightCredentials() async throws {
        try await withAppIncludingDB { app in
            let _ = try await createTestUser(on: app.db, roles: [.admin])

            let body = LoginController.LoginRequest(email: "test@mail.com", password: "Admin12345@")

            try await app.testing().test(.POST, "login", beforeRequest: { req in
                try req.content.encode(body)
            }, afterResponse: { res async throws in
                #expect(res.status == .ok)  

                var buffer = res.body // Copy the body into a var

                // Decode the JSON from the body returned by the login route
                guard let body = try buffer.readJSONDecodable(LoginController.LoginResponse.self, length: buffer.readableBytes) else {
                    throw Abort(.internalServerError)
                }

                // Try to verify the JWT (if it's not a valid one, it will fail
                let _ = try await app.jwt.keys.verify(body.token, as: TokenPayload.self)
            })
        }
    }

    @Test("Test new account route with a bad password")
    func testNewAccountWithBadPassword() async throws {
        try await withAppIncludingDB { app in
            let token = try await createTestUserAndGetJWT(app: app, roles: [.new_account])

            let body = LoginController.ModifyNewAccountRequest(new_email: "test@mail.com", new_password: "NotSecureEnough")

            try await app.testing().test(.POST, "modify-new-account/en", beforeRequest: { req in
                try req.content.encode(body)
                req.headers.add(name: "authorization", value: "Bearer \(token)")
            }, afterResponse: { res async throws in
                #expect(res.status == .badRequest)
            })
        }
    }

    @Test("Test new account route with an already existing email")
    func testNewAccountWithExistingEmail() async throws {
        try await withAppIncludingDB { app in
            let token = try await createTestUserAndGetJWT(app: app, roles: [.new_account])

            let body = LoginController.ModifyNewAccountRequest(new_email: "email@mail.com", new_password: "SecureEnough1")

            try await app.testing().test(.POST, "modify-new-account/en", beforeRequest: { req in
                try req.content.encode(body)
                req.headers.add(name: "authorization", value: "Bearer \(token)")
            }, afterResponse: { res async throws in
                #expect(res.status == .conflict)
            })
        }
    }

    @Test("Test new account route with right values and same email")
    func testNewAccountWithRightValuesSameEmail() async throws {
        try await withAppIncludingDB { app in
            let token = try await createTestUserAndGetJWT(app: app, roles: [.new_account])

            let body = LoginController.ModifyNewAccountRequest(new_email: "test@mail.com", new_password: "SecureEnough1")

            try await app.testing().test(.POST, "modify-new-account/en", beforeRequest: { req in
                try req.content.encode(body)
                req.headers.add(name: "authorization", value: "Bearer \(token)")
            }, afterResponse: { res async throws in
                #expect(res.status == .ok)
            })
        }
    }

    @Test("Test new account route with right values another email")
    func testNewAccountWithRightValuesOtherEmail() async throws {
        try await withAppIncludingDB { app in
            let token = try await createTestUserAndGetJWT(app: app, roles: [.new_account])

            let body = LoginController.ModifyNewAccountRequest(new_email: "newemail@mail.com", new_password: "SecureEnough1")

            try await app.testing().test(.POST, "modify-new-account/en", beforeRequest: { req in
                try req.content.encode(body)
                req.headers.add(name: "authorization", value: "Bearer \(token)")
            }, afterResponse: { res async throws in
                #expect(res.status == .ok)
            })
        }
    }

    @Test("Test verify email route with a wrong code")
    func testVerifyEmailWithWrongCode() async throws {
        try await withAppIncludingDB { app in
            let token = try await createTestUserAndGetJWT(app: app, roles: [.unverified_email])

            let body = LoginController.verifyEmailRequest(code: "aaaaaa")

            try await app.testing().test(.POST, "verify-email", beforeRequest: { req in
                try req.content.encode(body)
                req.headers.add(name: "authorization", value: "Bearer \(token)")
            }, afterResponse: { res async throws in
                #expect(res.status == .badRequest)
            })
        }
    }

    @Test("Test verify email route with a right code")
    func testVerifyEmailWithRightCode() async throws {
        try await withAppIncludingDB { app in
            let token = try await createTestUserAndGetJWT(app: app, roles: [.unverified_email])

            let body = LoginController.verifyEmailRequest(code: "secretcode")

            try await app.testing().test(.POST, "verify-email", beforeRequest: { req in
                try req.content.encode(body)
                req.headers.add(name: "authorization", value: "Bearer \(token)")
            }, afterResponse: { res async throws in
                #expect(res.status == .ok)
            })
        }
    }

}
