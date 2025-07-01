//
//  VerifyEmailViewModel.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 30/06/2025.
//

import Foundation
internal import Combine

@MainActor
class VerifyEmailViewModel: ObservableObject {
    @Published var code: String = ""
    @Published var error: VerifyEmailError? = nil
    
    
    
    func verifyEmail () async -> String? {
        LoadingManager.shared.isLoading = true;
        
        do {
            guard let url = URL(string: "\(Config.apiUrl)/verify-email") else {
                throw VerifyEmailError.unknown
            }
            var request = URLRequest(url: url)
            request.httpMethod = "POST"
            request.addValue("application/json", forHTTPHeaderField: "Content-Type")
            request.addValue("Bearer \(AuthManager.shared.token!)", forHTTPHeaderField: "Authorization")
            
            let body = VerifyEmailRequest(code: self.code)
            request.httpBody = try JSONEncoder().encode(body)
            
            let (_, response) = try await URLSession.shared.data(for: request)
            
            guard let httpResponse = response as? HTTPURLResponse else {
                throw URLError(.badServerResponse)
            }
            
            guard (200...299).contains(httpResponse.statusCode) else {
                if httpResponse.statusCode == 401 {
                    // If we have a 401 status code, we disconnect the user and display a flash message
                    throw URLError(.clientCertificateRejected)
                }
                // If we have a 400 status code, it means that the code isn't valid
                if httpResponse.statusCode == 400 {
                    throw VerifyEmailError.invalidCode
                }
                throw URLError(.badServerResponse)
            }
        } catch {
            switch error {
            case VerifyEmailError.invalidCode:
                self.error = VerifyEmailError.invalidCode
            case URLError.clientCertificateRejected:
                self.error = VerifyEmailError.unknown // We put the error here, in case the token is valid and something unknown appened
                await AuthManager.shared.verifyToken() // We let it verify that the token is really valid, if it is not, the user will be disconnected and put on the login page
            default:
                print(error)
                self.error = VerifyEmailError.unknown
            }
            self.code = ""
            LoadingManager.shared.isLoading = false
        }
        
        await AuthManager.shared.verifyToken()
        return nil
    }
}
