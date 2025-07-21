//
//  deleteUser.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 21/07/2025.
//

import Foundation

enum DeleteUserError: Error {
    case unknown
    
    var localizationKey: String {
        switch self {
        case .unknown: return "error_unknown"
        }
    }
}
