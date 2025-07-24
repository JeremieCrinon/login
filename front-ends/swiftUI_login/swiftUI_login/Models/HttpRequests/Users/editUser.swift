//
//  editUser.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 23/07/2025.
//

import Foundation

struct EditUserEmailRequest: Codable {
    let email: String
}

struct EditUserRolesRequest: Codable {
    let roles: [String]
}

enum EditUserError: Error {
    case emailAlreadyTaken
    case unknown
    
    var localizationKey: String {
        switch self {
        case .unknown: return "error_unknown"
        case .emailAlreadyTaken: return "new_user_email_taken"
        }
    }
}
