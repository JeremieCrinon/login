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
