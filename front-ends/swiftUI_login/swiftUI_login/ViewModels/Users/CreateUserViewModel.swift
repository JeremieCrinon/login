//
//  CreateUserViewModel.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 10/07/2025.
//

import Foundation
internal import Combine

@MainActor
class CreateUserViewModel: ObservableObject {
    @Published var opended: Bool = false
    
    @Published var email: String = ""
    @Published var roles: [String: Bool] = [:]
    @Published var locale: String = Bundle.main.localizations[0]
    @Published var error: CreateUserError? = nil
    @Published var validationError: Validators.ValidationError? = nil
    
    func createUser() async -> Void {
        LoadingManager.shared.isLoading = true;
        self.error = nil
        self.validationError = nil
        
        do {
            try Validators.isValidEmail(self.email)
            
            
            guard let url = URL(string: "\(Config.apiUrl)/users/new/\(self.locale)") else {
                throw CreateUserError.unknown;
            }
            var request = URLRequest(url: url)
            request.httpMethod = "POST"
            request.addValue("application/json", forHTTPHeaderField: "Content-Type")
            request.addValue("Bearer \(AuthManager.shared.token!)", forHTTPHeaderField: "Authorization")
            
            var roles: Array<String> {
                self.roles.filter { $0.value == true }.map { $0.key }
            }
            
            let body = CreateUserRequest(email: self.email, roles: roles)
            request.httpBody = try JSONEncoder().encode(body)
            
            let (_, response) = try await URLSession.shared.data(for: request)
            
            guard let httpResponse = response as? HTTPURLResponse else {
                throw URLError(.badServerResponse)
            }
            
            guard (200...299).contains(httpResponse.statusCode) else {
                // If we have a 409 code, it means the new email adress the user sent is already taken by another account.
                if httpResponse.statusCode == 409 {
                    throw CreateUserError.emailAlreadyTaken
                }
                throw URLError(.badServerResponse)
            }
            self.opended = false
            self.email = ""
            
            for key in self.roles.keys {
                self.roles[key] = false
            }
            
            LoadingManager.shared.isLoading = false
            
            
        } catch {
            switch error {
                case let validationError as Validators.ValidationError:
                    self.validationError = validationError
                case CreateUserError.emailAlreadyTaken:
                    self.error = CreateUserError.emailAlreadyTaken
                default:
                    print(error)
                    self.error = CreateUserError.unknown
            }
            
            LoadingManager.shared.isLoading = false
        }
    }
    
    func getRoles() async -> Void {
        do {
            
            guard let url = URL(string: "\(Config.apiUrl)/users/list-roles") else {
                throw GetRolesError.unknown;
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
            
            let decoded = try JSONDecoder().decode(GetRolesResponse.self, from: data)
            
            for role in decoded.roles {
                self.roles[role] = false
            }
            
        } catch {
            print(error)
            
            self.opended = false // Close the sheet because we couldn't fetch the roles list
            
            MessagesManager.shared.setMessage(title: MessageTitle.error, message: MessageDesc.error)
            return;
        }
    }
    
}
