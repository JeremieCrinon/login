@testable import App
import VaporTesting
import Testing
import JWTKit
import struct Foundation.UUID

@Suite("users routes tests")
struct UsersRoutesTests {
    @Test("Test user creation with an incorrect email")
    func testUserCreationWithIncorrectEmail() async throws {
        try await withAppIncludingDB { app in
            let token = try await createTestUserAndGetJWT(app: app, roles: [.edit_users])

            let body = UsersController.createUserRequest(
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

            let body = UsersController.createUserRequest(
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

            let body = UsersController.createUserRequest(
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

    @Test("Test user deletion")
    func testUserDeletion() async throws {
        try await withAppIncludingDB { app in
            let token = try await createTestUserAndGetJWT(app: app, roles: [.admin])
            let user = try await createTestUser(on: app.db, roles: [], email: "delete@mail.com")
            let id: UUID = user.id!

            try await app.testing().test(.DELETE, "users/\(id)", beforeRequest: { req in
                req.headers.add(name: "authorization", value: "Bearer \(token)")
            }, afterResponse: { res async throws in
                #expect(res.status == .ok)
            })
        }
    }

    @Test("Test user email editing with an already existing email")
    func testUserEmailEditingExistingEmail() async throws {
        try await withAppIncludingDB { app in
            let token = try await createTestUserAndGetJWT(app: app, roles: [.admin])
            let user = try await createTestUser(on: app.db, roles: [], email: "editemail@mail.com")
            let id: UUID = user.id!
            
            let body = UsersController.editUserEmailRequest(
                email: "test@mail.com" // The email already taken by the user we got the JWT from
            )

            try await app.testing().test(.PUT, "users/\(id)/email/en", beforeRequest: { req in 
                try req.content.encode(body)
                req.headers.add(name: "authorization", value: "Bearer \(token)")
            }, afterResponse: { res async throws in 
                #expect(res.status == .conflict)
            })
        }
    }

    @Test("Test user email editing with right values")
    func testUserEmailEditingRightValues() async throws {
        try await withAppIncludingDB { app in
            let token = try await createTestUserAndGetJWT(app: app, roles: [.admin])
            let user = try await createTestUser(on: app.db, roles: [], email: "editemail@mail.com")
            let id: UUID = user.id!
            
            let body = UsersController.editUserEmailRequest(
                email: "unique@mail.com"
            )

            try await app.testing().test(.PUT, "users/\(id)/email/en", beforeRequest: { req in 
                try req.content.encode(body)
                req.headers.add(name: "authorization", value: "Bearer \(token)")
            }, afterResponse: { res async throws in 
                #expect(res.status == .ok)
            })
        }
    }

    @Test("Test user roles editing")
    func testUserRolesEditing() async throws {
        try await withAppIncludingDB { app in 
            let token = try await createTestUserAndGetJWT(app: app, roles: [.admin])
            let user = try await createTestUser(on: app.db, roles: [], email: "editemail@mail.com")
            let id: UUID = user.id!
            
            let body = UsersController.editUserRolesRequest(
                roles: [.user]
            )

            try await app.testing().test(.PUT, "users/\(id)/roles", beforeRequest: { req in 
                try req.content.encode(body)
                req.headers.add(name: "authorization", value: "Bearer \(token)")
            }, afterResponse: { res async throws in 
                #expect(res.status == .ok)
            })

        }
    }
}
