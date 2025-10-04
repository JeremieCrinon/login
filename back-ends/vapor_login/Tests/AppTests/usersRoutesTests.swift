@testable import App
import VaporTesting
import Testing
import JWTKit

@Suite("users routes tests")
struct UsersRoutesTests {
    @Test("Test user creation with an incorrect email")
    func testUserCreationWithIncorrectEmail() async throws {
        try await withAppIncludingDB { app in
            let token = try await createTestUserAndGetJWT(app: app, roles: [.edit_users])

            let body = UsersController.CreateUserRequest(
                email: "email",
                roles: [.admin]
            )

            try await app.testing().test(.POST, "users/new/en", beforeRequest: { req in
                try req.content.encode(body)
                req.headers.add(name: "authorization", value: "Bearer \(token)")
            }, afterResponse: { res async throws in
                #expect(res.status == .badRequest)
            })
        }
    }

    @Test("Test user creation with already existing email")
    func testUserCreationWithAlreadyExistingEmail() async throws {
        try await withAppIncludingDB { app in
            let token = try await createTestUserAndGetJWT(app: app, roles: [.edit_users])

            let body = UsersController.CreateUserRequest(
                email: "test@mail.com",
                roles: [.admin]
            )

            try await app.testing().test(.POST, "users/new/en", beforeRequest: { req in
                try req.content.encode(body)
                req.headers.add(name: "authorization", value: "Bearer \(token)")
            }, afterResponse: { res async throws in
                #expect(res.status == .conflict)
            })
        }
    }

    @Test("Test user creation with right values")
    func testUserCreationWithRightValues() async throws {
        try await withAppIncludingDB { app in
            let token = try await createTestUserAndGetJWT(app: app, roles: [.admin])

            let body = UsersController.CreateUserRequest(
                email: "user@mail.com",
                roles: [.admin]
            )

            try await app.testing().test(.POST, "users/new/en", beforeRequest: { req in
                try req.content.encode(body)
                req.headers.add(name: "authorization", value: "Bearer \(token)")
            }, afterResponse: { res async throws in
                #expect(res.status == .ok)
            })
        }
    }
}
