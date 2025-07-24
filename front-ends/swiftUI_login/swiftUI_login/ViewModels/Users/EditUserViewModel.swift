//
//  EditUserViewModel.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 23/07/2025.
//

import Foundation
internal import Combine

@MainActor
class EditUserViewModel: ObservableObject {
    @Published var isOpened: Bool = false
    
    @Published var user: User = User(id: 1, email: "", roles: [], created_at: "2025-04-27T10:02:57.970461", updated_at: "2025-04-27T10:02:57.970461") // Put a default value of the user for the views, even tough it should never be displayed, we cannot put something nil here
    @Published var locale: String = Bundle.main.localizations[0]
    @Published var roles: [String] = []
    
    @Published var error: EditUserError? = nil
    @Published var validationError: Validators.ValidationError? = nil
    
    func editUserEmail () async -> Void {
        LoadingManager.shared.isLoading = true;
        self.error = nil
        self.validationError = nil
        
        do {
            try Validators.isValidEmail(self.user.email)
            
            
            guard let url = URL(string: "\(Config.apiUrl)/users/\(self.user.id)/email/\(locale)") else {
                throw EditUserError.unknown;
            }
            var request = URLRequest(url: url)
            request.httpMethod = "PUT"
            request.addValue("application/json", forHTTPHeaderField: "Content-Type")
            request.addValue("Bearer \(AuthManager.shared.token!)", forHTTPHeaderField: "Authorization")
            
        
            let body = EditUserEmailRequest(email: self.user.email)
            request.httpBody = try JSONEncoder().encode(body)
            
            let (_, response) = try await URLSession.shared.data(for: request)
            
            guard let httpResponse = response as? HTTPURLResponse else {
                throw URLError(.badServerResponse)
            }
            
            guard (200...299).contains(httpResponse.statusCode) else {
                print(httpResponse)
                print(httpResponse.statusCode)
                // If we have a 409 code, it means the new email adress the user sent is already taken by another account.
                if httpResponse.statusCode == 409 {
                    throw EditUserError.emailAlreadyTaken
                }
                throw URLError(.badServerResponse)
            }
            self.isOpened = false
            
            LoadingManager.shared.isLoading = false
            
            
        } catch {
            switch error {
                case let validationError as Validators.ValidationError:
                    self.validationError = validationError
                case EditUserError.emailAlreadyTaken:
                    self.error = EditUserError.emailAlreadyTaken
                default:
                    print(error)
                    self.error = EditUserError.unknown
            }
            
            LoadingManager.shared.isLoading = false
        }
    }
    
    func editUserRoles () async -> Void {
        LoadingManager.shared.isLoading = true;
        self.error = nil
        self.validationError = nil
        
        do {
            guard let url = URL(string: "\(Config.apiUrl)/users/\(self.user.id)/roles") else {
                throw EditUserError.unknown;
            }
            var request = URLRequest(url: url)
            request.httpMethod = "PUT"
            request.addValue("application/json", forHTTPHeaderField: "Content-Type")
            request.addValue("Bearer \(AuthManager.shared.token!)", forHTTPHeaderField: "Authorization")
            
            
            let body = EditUserRolesRequest(roles: self.user.roles)
            request.httpBody = try JSONEncoder().encode(body)
            
            let (_, response) = try await URLSession.shared.data(for: request)
            
            guard let httpResponse = response as? HTTPURLResponse else {
                throw URLError(.badServerResponse)
            }
            
            guard (200...299).contains(httpResponse.statusCode) else {
                throw URLError(.badServerResponse)
            }
            self.isOpened = false
            
            LoadingManager.shared.isLoading = false
            
            
        } catch {
            switch error {
                case let validationError as Validators.ValidationError:
                    self.validationError = validationError
                default:
                    print(error)
                    self.error = EditUserError.unknown
            }
            
            LoadingManager.shared.isLoading = false
        }
    }
    
    func getUser (userId: Int) async -> Void {
        LoadingManager.shared.isLoading = true
        
        do {
            guard let url = URL(string: "\(Config.apiUrl)/users/\(userId)") else {
                throw GetUserError.unknown;
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
            
            self.user = try JSONDecoder().decode(User.self, from: data) // Since it returns a user directly, we do not need a GetUserResponse
            
            self.isOpened = true
            LoadingManager.shared.isLoading = false
        } catch {
            // We do not display an error message here, we just don't open the sheet
            print(error)
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
            
            self.roles = decoded.roles
            
        } catch {
            print(error)
            
            self.isOpened = false // Close the sheet because we couldn't fetch the roles list
            
            MessagesManager.shared.setMessage(title: MessageTitle.error, message: MessageDesc.error)
            return;
        }
    }
}
