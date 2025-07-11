//
//  getRoles.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 11/07/2025.
//

import Foundation

enum GetRolesError: Error {
    case unknown
    
    var localizationKey: String {
        switch self {
        case .unknown: return "error_unknown"
        }
    }
}

struct GetRolesResponse: Codable {
    let roles: Array<String>
}
