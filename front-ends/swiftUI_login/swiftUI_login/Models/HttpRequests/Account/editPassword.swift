//
//  editPassword.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 02/07/2025.
//

import Foundation

struct EditPasswordRequest: Codable {
    let current_password: String
    let new_password: String
}

enum EditPasswordError: Error {
    case invalidCredentials
    case passwordConfirmNotMatching
    case unknown
    
    var localizationKey: String {
        switch self {
        case .passwordConfirmNotMatching: return "password_confirm_not_matching"
        case .invalidCredentials: return "edit_password_error_invalidCredentials"
        case .unknown: return "error_unknown"
        }
    }
}
