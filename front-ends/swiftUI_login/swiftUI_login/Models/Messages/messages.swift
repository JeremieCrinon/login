//
//  messages.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 30/06/2025.
//

import Foundation

enum MessageTitle {
    case newAccount
    case editedEmail
    case editedPassword
    case error
    
    var localizationKey: String {
        switch self {
        case .newAccount: return "new_account_success_title"
        case .editedEmail: return "edit_email_success_title"
        case .editedPassword: return "edit_password_success_title"
        case .error: return "error_title"
        }
    }
}

enum MessageDesc {
    case newAccount
    case editedEmail
    case editedPassword
    case error
    
    var localizationKey: String {
        switch self {
        case .newAccount: return "new_account_success_desc"
        case .editedEmail: return "edit_email_success_desc"
        case .editedPassword: return "edit_password_success_desc"
        case .error: return "error_desc"
        }
    }
}
