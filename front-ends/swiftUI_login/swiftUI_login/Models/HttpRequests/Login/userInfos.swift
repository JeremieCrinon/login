//
//  userInfos.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 28/06/2025.
//

import Foundation

enum UserInfosError: Error {
    case invalidURL
}

struct UserInfosResponse: Codable {
    let result: Bool
    let roles: Array<String>
    let user_mail: String
}
