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
}
