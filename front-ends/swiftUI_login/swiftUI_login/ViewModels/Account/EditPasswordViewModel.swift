//
//  EditPasswordViewModel.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 02/07/2025.
//

import Foundation
internal import Combine

@MainActor
class EditPasswordViewModel: ObservableObject {
    @Published var current_password: String = ""
    @Published var new_password: String = ""
    @Published var confirm_password: String = ""
    @Published var error: EditPasswordError? = nil
    @Published var validationError: Validators.ValidationError? = nil
    
    func EditPassword() async -> Void {
        LoadingManager.shared.isLoading = true
        
        do {
            self.error = nil
            self.validationError = nil
            
            if self.new_password != self.confirm_password {
                throw EditPasswordError.passwordConfirmNotMatching
            }
            
            try Validators.validatePassword(self.new_password)
            
            guard let url = URL(string: "\(Config.apiUrl)/edit-password") else {
                throw EditPasswordError.unknown;
            }
            var request = URLRequest(url: url)
            request.httpMethod = "POST"
            request.addValue("application/json", forHTTPHeaderField: "Content-Type")
            request.addValue("Bearer \(AuthManager.shared.token!)", forHTTPHeaderField: "Authorization")
            
            let body = EditPasswordRequest(current_password: self.current_password, new_password: self.new_password)
            request.httpBody = try JSONEncoder().encode(body)
            
            let (_, response) = try await URLSession.shared.data(for: request)
            
            guard let httpResponse = response as? HTTPURLResponse else {
                throw URLError(.badServerResponse)
            }
            
            guard (200...299).contains(httpResponse.statusCode) else {
                if httpResponse.statusCode == 401 {
                    throw EditPasswordError.invalidCredentials
                }
                throw URLError(.badServerResponse)
            }
            
            AuthManager.shared.logout()
            MessagesManager.shared.setMessage(title: MessageTitle.editedPassword, message: MessageDesc.editedPassword)
            LoadingManager.shared.isLoading = false
        } catch {
            switch error {
                case EditPasswordError.passwordConfirmNotMatching:
                    self.error = EditPasswordError.passwordConfirmNotMatching
                case EditPasswordError.invalidCredentials:
                    self.error = EditPasswordError.invalidCredentials
                case let validationError as Validators.ValidationError:
                    self.validationError = validationError
                default:
                    print(error)
                    self.error = EditPasswordError.unknown
            }
            
            LoadingManager.shared.isLoading = false
        }
        
    }
}
