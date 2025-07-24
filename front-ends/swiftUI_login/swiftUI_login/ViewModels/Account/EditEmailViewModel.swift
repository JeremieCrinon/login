//
//  EditEmailViewModel.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 01/07/2025.
//

import Foundation
internal import Combine

@MainActor
class EditEmailViewModel: ObservableObject {
    @Published var email: String = AuthManager.shared.user_email
    @Published var password: String = ""
    @Published var error: EditEmailError? = nil
    @Published var validationError: Validators.ValidationError? = nil
    
    func editEmail() async -> Void {
        LoadingManager.shared.isLoading = true
        
        do {
            try Validators.isValidEmail(self.email)
            
            guard let url = URL(string: "\(Config.apiUrl)/edit-email/\(Locale.current.language.languageCode?.identifier ?? "en")") else {
                throw EditEmailError.unknown;
            }
            var request = URLRequest(url: url)
            request.httpMethod = "POST"
            request.addValue("application/json", forHTTPHeaderField: "Content-Type")
            request.addValue("Bearer \(AuthManager.shared.token!)", forHTTPHeaderField: "Authorization")
            
            let body = EditEmailRequest(new_email: self.email, password: self.password)
            request.httpBody = try JSONEncoder().encode(body)
            
            let (_, response) = try await URLSession.shared.data(for: request)
            
            guard let httpResponse = response as? HTTPURLResponse else {
                throw URLError(.badServerResponse)
            }
            
            guard (200...299).contains(httpResponse.statusCode) else {
                if httpResponse.statusCode == 401 {
                    throw EditEmailError.invalidCredentials
                }
                throw URLError(.badServerResponse)
            }
            
            AuthManager.shared.logout()
            MessagesManager.shared.setMessage(title: MessageTitle.editedEmail, message: MessageDesc.editedEmail)
            LoadingManager.shared.isLoading = false
        } catch {
            switch error {
                case EditEmailError.invalidCredentials:
                    self.error = EditEmailError.invalidCredentials
                case let validationError as Validators.ValidationError:
                    self.validationError = validationError
                default:
                    print(error)
                    self.error = EditEmailError.unknown
            }
            
            LoadingManager.shared.isLoading = false
        }
    }
}
