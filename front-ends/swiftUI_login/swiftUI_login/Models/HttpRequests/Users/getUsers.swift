//
//  getUsers.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 13/07/2025.
//

import Foundation

enum GetUsersError: Error {
    case unknown
    
    var localizationKey: String {
        switch self {
        case .unknown: return "error_unknown"
        }
    }
}



struct GetUsersResponse: Codable {
    let users: [UserShort]
}
