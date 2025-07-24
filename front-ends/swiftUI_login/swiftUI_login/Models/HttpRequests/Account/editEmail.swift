//
//  editEmail.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 01/07/2025.
//

import Foundation

struct EditEmailRequest: Codable {
    let new_email: String
    let password: String
}

enum EditEmailError: Error {
    case invalidCredentials
    case unknown
    
    var localizationKey: String {
        switch self {
        case .invalidCredentials: return "edit_email_error_invalidCredentials"
        case .unknown: return "error_unknown"
        }
    }
}
