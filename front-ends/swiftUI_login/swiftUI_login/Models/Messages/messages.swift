//
//  messages.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 30/06/2025.
//

import Foundation

enum MessageTitle {
    case newAccount
    
    var localizationKey: String {
        switch self {
        case .newAccount: return "new_account_success_title"
        }
    }
}

enum MessageDesc {
    case newAccount
    
    var localizationKey: String {
        switch self {
        case .newAccount: return "new_account_success_desc"
        }
    }
}
