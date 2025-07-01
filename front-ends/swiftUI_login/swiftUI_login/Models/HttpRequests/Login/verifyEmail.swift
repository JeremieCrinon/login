//
//  verifyEmail.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 30/06/2025.
//

import Foundation

struct VerifyEmailRequest: Codable {
    let code: String
}

enum VerifyEmailError: Error {
    case invalidCode
    case unknown
    
    var localizationKey: String {
        switch self {
        case .invalidCode: return "invalid_email_code"
        case .unknown: return "error_unknown"
        }
    }
}
