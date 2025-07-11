//
//  createUser.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 10/07/2025.
//

import Foundation

struct CreateUserRequest: Codable {
    let email: String
    let roles: Array<String>
}

enum CreateUserError: Error {
    case emailAlreadyTaken
    case unknown
    
    var localizationKey: String {
        switch self {
        case .unknown: return "error_unknown"
        case .emailAlreadyTaken: return "new_user_email_taken"
        }
    }
}

