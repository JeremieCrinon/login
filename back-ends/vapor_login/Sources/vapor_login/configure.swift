import NIOSSL
import Fluent
import FluentPostgresDriver
import Vapor
import Mailgun
import JWT
import Crypto

extension MailgunDomain {
    static var domain1: MailgunDomain { .init(Environment.get("MAILGUN_DOMAIN") ?? "mg.example.com", .eu) }
}

// configures your application
public func configure(_ app: Application) async throws {
    // uncomment to serve files from /Public folder
    // app.middleware.use(FileMiddleware(publicDirectory: app.directory.publicDirectory))

    app.databases.use(DatabaseConfigurationFactory.postgres(configuration: .init(
        hostname: Environment.get("DATABASE_HOST") ?? "localhost",
        port: Environment.get("DATABASE_PORT").flatMap(Int.init(_:)) ?? SQLPostgresConfiguration.ianaPortNumber,
        username: Environment.get("DATABASE_USERNAME") ?? "vapor_username",
        password: Environment.get("DATABASE_PASSWORD") ?? "vapor_password",
        database: Environment.get("DATABASE_NAME") ?? "vapor_database",
        tls: .prefer(try .init(configuration: .clientDefault)))
    ), as: .psql)

    app.passwords.use(.bcrypt)

    app.mailgun.configuration = .environment

    app.mailgun.defaultDomain = .domain1

    app.migrations.add(CreateUser())

    let secret = Environment.get("JWT_SECRET") ?? "secret"
    await app.jwt.keys.add(hmac: HMACKey(from: Array(secret.utf8)), digestAlgorithm: .sha256)

    // register routes
    try routes(app)
}
