//
//  LoginViewModel.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 29/06/2025.
//

import Foundation
import KeychainSwift
internal import Combine

@MainActor
class LoginViewModel: ObservableObject {
    @Published var email: String = ""
    @Published var password: String = ""
    @Published var error: String? = nil
    
    let keychain = KeychainSwift();
    
    func login() async -> Void {
        LoadingManager.shared.isLoading = true;
        do {
            guard let url = URL(string: "\(Config.apiUrl)/login") else {
                throw LoginError.invalidURL;
            }
            var request = URLRequest(url: url)
            request.httpMethod = "POST"
            request.addValue("application/json", forHTTPHeaderField: "Content-Type")
            
            let body = LoginRequest(email: self.email, password: self.password)
            request.httpBody = try JSONEncoder().encode(body)
            
            let (data, response) = try await URLSession.shared.data(for: request)
            
            guard let httpResponse = response as? HTTPURLResponse else {
                throw URLError(.badServerResponse)
            }
            
            guard (200...299).contains(httpResponse.statusCode) else {
                if httpResponse.statusCode == 400 {
                    throw LoginError.invalidCredentials
                }
                throw URLError(.badServerResponse)
            }
            
            let decoded = try JSONDecoder().decode(LoginResponse.self, from: data)
            
            self.keychain.set(decoded.token, forKey: "token");
            
            await AuthManager.shared.verifyToken();
        } catch {
            switch error {
            case LoginError.invalidCredentials:
                LoadingManager.shared.isLoading = false;
                self.error = "Invalid credentials"
            default:
                print(error)
                LoadingManager.shared.isLoading = false;
                self.error = "An error occurred, please try again later."
            }
            
        }
        
    }
}
