//
//  DeleteUserViewModel.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 21/07/2025.
//

import Foundation
internal import Combine

@MainActor
class DeleteUserViewModel: ObservableObject {
    @Published var error: DeleteUserError? = nil
    
    func deleteUser(user: UserShort) async -> Void {
        do {
            guard let url = URL(string: "\(Config.apiUrl)/users/\(user.id)") else {
                throw DeleteUserError.unknown
            }
            var request = URLRequest(url: url)
            request.httpMethod = "DELETE"
            request.addValue("Bearer \(AuthManager.shared.token!)", forHTTPHeaderField: "Authorization")
            
            let (_, response) = try await URLSession.shared.data(for: request)
            
            guard let httpResponse = response as? HTTPURLResponse else {
                throw URLError(.badServerResponse)
            }
            
            guard (200...299).contains(httpResponse.statusCode) else {
                throw URLError(.badServerResponse)
            }
            
        } catch {
            print(error)
            self.error = DeleteUserError.unknown
        }
    }
}
