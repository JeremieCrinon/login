//
//  NewAccountViewModel.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 30/06/2025.
//

import Foundation
internal import Combine

@MainActor
class NewAccountViewModel: ObservableObject {
    @Published var email: String = AuthManager.shared.user_email
    @Published var password: String = ""
    @Published var confirmPassword: String = ""
    @Published var error: NewAccountError? = nil
    @Published var validationError: Validators.ValidationError? = nil
    
    func modifyNewAccount() async -> Void {
        
        LoadingManager.shared.isLoading = true;
        self.error = nil
        self.validationError = nil
        
        do {
            if self.password != self.confirmPassword {
                throw NewAccountError.passwordConfirmNotMatching
            }
            
            try Validators.isValidEmail(self.email)
            try Validators.validatePassword(self.password)
            
            
            guard let url = URL(string: "\(Config.apiUrl)/modify-new-account/\(Locale.current.language.languageCode?.identifier ?? "en")") else {
                throw NewAccountError.unknown;
            }
            var request = URLRequest(url: url)
            request.httpMethod = "POST"
            request.addValue("application/json", forHTTPHeaderField: "Content-Type")
            request.addValue("Bearer \(AuthManager.shared.token!)", forHTTPHeaderField: "Authorization")
            
            let body = NewAccountRequest(new_email: self.email, new_password: self.password)
            request.httpBody = try JSONEncoder().encode(body)
            
            let (_, response) = try await URLSession.shared.data(for: request)
            
            guard let httpResponse = response as? HTTPURLResponse else {
                throw URLError(.badServerResponse)
            }
            
            guard (200...299).contains(httpResponse.statusCode) else {
                print(httpResponse.statusCode)
                if httpResponse.statusCode == 401 {
                    throw URLError(.clientCertificateRejected)
                }
                // If we have a 409 code, it means the new email adress the user sent is already taken by another account.
                if httpResponse.statusCode == 409 {
                    throw NewAccountError.emailAlreadyTaken
                }
                throw URLError(.badServerResponse)
            }
            
            AuthManager.shared.logout()
    //        MessagesManager.shared.setMessage(title: "Success!",message: "Your informations have successfully been updated, please login with those new credentials.")
            LoadingManager.shared.isLoading = false
            
            
        } catch {
            switch error {
                case NewAccountError.passwordConfirmNotMatching:
                    self.error = NewAccountError.passwordConfirmNotMatching
                case let validationError as Validators.ValidationError:
                    self.validationError = validationError
                case URLError.clientCertificateRejected:
                    self.error = NewAccountError.unknown // We put the error here, in case the token is valid and something unknown appened
                    await AuthManager.shared.verifyToken() // We let it verify that the token is really valid, if it is not, the user will be disconnected and put on the login page
                case NewAccountError.emailAlreadyTaken:
                    self.error = NewAccountError.emailAlreadyTaken
                default:
                    print(error)
                    self.error = NewAccountError.unknown
            }
            
            LoadingManager.shared.isLoading = false
        }
    }
}
