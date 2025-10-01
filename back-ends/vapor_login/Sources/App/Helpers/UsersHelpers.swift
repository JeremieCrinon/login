import Crypto
import Foundation

func generatePassword(length: Int) -> String {
    let characters = Array("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")
    var password = ""

    for _ in 0..<length {
        let i = Int.random(in: 0..<characters.count)
        password.append(characters[i])
    }

    return password
}
