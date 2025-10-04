@testable import App
import VaporTesting
import Testing
import JWTKit

@Suite("auth middleware tests")
struct AuthMiddlewareTests {
    @Test("Test protected route without a token")
    func testProtectedRouteWithoutToken() async throws {
        try await withAppIncludingDB { app in
            // Call a route that requires the edit_users role without sending a JWT
            try await app.testing().test(.GET, "edit_users") { res async in
                #expect(res.status == .unauthorized)
            }
        }
    }

    @Test("Test protected route without the right role")
    func testProtectedRouteWithoutRole() async throws {
        try await withAppIncludingDB { app in
            let token = try await createTestUserAndGetJWT(app: app, roles: [.user])

            try await app.testing().test(.GET, "edit_users", beforeRequest: { req in
                req.headers.add(name: "authorization", value: "Bearer \(token)")
            }, afterResponse: { res async throws in
                #expect(res.status == .unauthorized)
            })
        }
    }


    @Test("Test protected route with new_account role")
    func testProtectedRouteWithNewAccount() async throws {
        try await withAppIncludingDB { app in
            let token = try await createTestUserAndGetJWT(app: app, roles: [.new_account, .edit_users])

            try await app.testing().test(.GET, "edit_users", beforeRequest: { req in
                req.headers.add(name: "authorization", value: "Bearer \(token)")
            }, afterResponse: { res async throws in
                #expect(res.status == .unauthorized)
            })
        }
    }

    @Test("Test protected route with right role")
    func testProtectedRouteWithRightRole() async throws {
        try await withAppIncludingDB { app in
            let token = try await createTestUserAndGetJWT(app: app, roles: [.edit_users])

            try await app.testing().test(.GET, "edit_users", beforeRequest: { req in
                req.headers.add(name: "authorization", value: "Bearer \(token)")
            }, afterResponse: { res async throws in
                #expect(res.status == .ok)
            })
        }
    }

    @Test("Test protected route with admin role")
    func testProtectedRouteWithAdminRole() async throws {
        try await withAppIncludingDB { app in
            let token = try await createTestUserAndGetJWT(app: app, roles: [.admin])

            try await app.testing().test(.GET, "edit_users", beforeRequest: { req in
                req.headers.add(name: "authorization", value: "Bearer \(token)")
            }, afterResponse: { res async throws in
                #expect(res.status == .ok)
            })
        }
    }
}
