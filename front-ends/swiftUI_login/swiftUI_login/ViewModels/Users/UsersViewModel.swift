//
//  UsersViewModel.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 13/07/2025.
//

import Foundation
internal import Combine

@MainActor
class UsersViewModel: ObservableObject {
    @Published var users: [UserShort] = []
    
    func getUsers() async -> Void {
        do {
            guard let url = URL(string: "\(Config.apiUrl)/users") else {
                throw GetUsersError.unknown;
            }
            var request = URLRequest(url: url)
            request.httpMethod = "GET"
            request.addValue("Bearer \(AuthManager.shared.token!)", forHTTPHeaderField: "Authorization")
            
            let (data, response) = try await URLSession.shared.data(for: request)
            
            guard let httpResponse = response as? HTTPURLResponse else {
                throw URLError(.badServerResponse)
            }
            
            guard (200...299).contains(httpResponse.statusCode) else {
                throw URLError(.badServerResponse)
            }
            
            let decoded = try JSONDecoder().decode(GetUsersResponse.self, from: data)
            
            // Reset the users var to not have duplicates in case it is called more than once
            self.users = []
            
            for user in decoded.users {
                self.users.append(user)
            }
        } catch {
            print(error)
        }
    }
}
