//
//  getUser.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 23/07/2025.
//

import Foundation

enum GetUserError: Error {
    case unknown
    
    var localizationKey: String {
        switch self {
        case .unknown: return "error_unknown"
        }
    }
}
