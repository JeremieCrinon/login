//
//  Validators.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 30/06/2025.
//

import Foundation

struct Validators {
    enum ValidationError: Error {
        case tooShort
        case noUppercase
        case noLowercase
        case noDigit
        case invalidEmail

        var localizationKey: String {
            switch self {
            case .tooShort: return "password_too_short"
            case .noUppercase: return "password_no_uppercase"
            case .noLowercase: return "password_no_lowercase"
            case .noDigit: return "password_no_digit"
            case .invalidEmail: return "email_not_valid"
            }
        }
    }
    
    static func isValidEmail(_ email: String) throws {
        let regex = #"^[A-Z0-9a-z._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$"#
        if !NSPredicate(format: "SELF MATCHES %@", regex).evaluate(with: email) {
            throw ValidationError.invalidEmail
        }
    }

    static func validatePassword(_ pw: String) throws {
        if pw.count < 8 {
            throw ValidationError.tooShort
        }
        if !pw.contains(where: \.isUppercase) {
            throw ValidationError.noUppercase
        }
        if !pw.contains(where: \.isLowercase) {
            throw ValidationError.noLowercase
        }
        if !pw.contains(where: \.isNumber) {
            throw ValidationError.noDigit
        }
    }
}
