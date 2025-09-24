import Vapor


struct UsersController: RouteCollection {

    func boot(routes: any RoutesBuilder) throws {
        let users = routes.grouped("users")
        
        users.post("new", ":lang", use: create)
    }

    struct CreateUserRequest: Content, Validatable {
        let email: String
        let roles: [Role]

        static func validations(_ validations: inout Validations) {
            validations.add("email", as: String.self, is: .email)
        }
    }

    func create(req: Request) async throws -> String {

        try CreateUserRequest.validate(content: req)
        let input = try req.content.decode(CreateUserRequest.self)

        print(input)

        print(input.email)

        let password = generatePassword(length: 12)

        let password_hash = try req.password.hash(password)

        let new_user = User(email: input.email, password: password_hash, roles: input.roles)

        try await new_user.save(on: req.db)

        return password
    }
}
