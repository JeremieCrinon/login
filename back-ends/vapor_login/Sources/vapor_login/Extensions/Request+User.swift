import Vapor

private struct UserKey: StorageKey {
    typealias Value = User
}

extension Request {
    var user: User? {
        get { storage[UserKey.self] }
        set { storage[UserKey.self] = newValue }
    }
}
