import NIOSSL
import Fluent
import FluentPostgresDriver
import FluentSQLiteDriver
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
    if app.environment == .testing {
        app.databases.use(.sqlite(.memory), as: .sqlite)// Use sqlite in memory to prevent conflicts when running tests in paralel
    } else {
        let useTLS = (Environment.get("DATABASE_USE_TLS") ?? "true").lowercased() == "true"

        app.databases.use(DatabaseConfigurationFactory.postgres(configuration: .init(
            hostname: Environment.get("DATABASE_HOST") ?? "localhost",
            port: Environment.get("DATABASE_PORT").flatMap(Int.init(_:)) ?? SQLPostgresConfiguration.ianaPortNumber,
            username: Environment.get("DATABASE_USERNAME") ?? "vapor_username",
            password: Environment.get("DATABASE_PASSWORD") ?? "vapor_password",
            database: Environment.get("DATABASE_NAME") ?? "vapor_database",
            tls: useTLS ? .prefer(try .init(configuration: .clientDefault)) : .disable)
        ), as: .psql)
    }

    app.passwords.use(.bcrypt)

    app.mailgun.configuration = .environment

    app.mailgun.defaultDomain = .domain1

    app.migrations.add(CreateUser())

    let secret = Environment.get("JWT_SECRET") ?? "secret"
    await app.jwt.keys.add(hmac: HMACKey(from: Array(secret.utf8)), digestAlgorithm: .sha256)

    let allowedOrigins = Environment.get("ALLOWED_ORIGINS")?
    .split(separator: ",")
    .map { String($0).trimmingCharacters(in: .whitespaces) } ?? []

    let corsConfiguration = CORSMiddleware.Configuration(
        allowedOrigin: .any(allowedOrigins),
        allowedMethods: [.GET, .POST, .PUT, .OPTIONS, .DELETE, .PATCH],
        allowedHeaders: [.accept, .authorization, .contentType, .origin, .xRequestedWith, .userAgent, .accessControlAllowOrigin]
    )

    let cors = CORSMiddleware(configuration: corsConfiguration)
    app.middleware.use(cors, at: .beginning)

    // register routes
    try routes(app)
}
