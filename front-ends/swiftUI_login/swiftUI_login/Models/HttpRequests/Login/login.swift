//
//  login.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 29/06/2025.
//

import Foundation

struct LoginRequest: Codable {
    let email: String
    let password: String
}

enum LoginError: Error {
    case invalidCredentials
    case unknown
    
    var localizationKey: String {
        switch self {
        case .invalidCredentials: return "login_error_invalidCredentials"
        case .unknown: return "error_unknown"
        }
    }
}



struct LoginResponse: Codable {
    let token: String
}

