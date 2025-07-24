//
//  users.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 13/07/2025.
//

import Foundation

struct UserShort: Codable, Hashable {
    let id: Int
    let email: String
    let roles: [String]
}

struct User: Codable, Hashable {
    let id: Int
    var email: String
    var roles: [String]
    let created_at: String
    let updated_at: String
}
