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
    case invalidURL
    case invalidCredentials
}

struct LoginResponse: Codable {
    let token: String
}

