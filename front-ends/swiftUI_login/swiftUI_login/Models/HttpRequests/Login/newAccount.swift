//
//  newAccount.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 30/06/2025.
//

import Foundation

struct NewAccountRequest: Codable {
    let new_email: String
    let new_password: String
}

enum NewAccountError: Error {
    case passwordConfirmNotMatching
    case emailAlreadyTaken
    case unknown
    
    var localizationKey: String {
        switch self {
        case .passwordConfirmNotMatching: return "password_confirm_not_matching"
        case .emailAlreadyTaken: return "email_already_taken"
        case .unknown: return "error_unknown"
        }
    }
}
