//
//  AuthManager.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 28/06/2025.
//

import Foundation
import SwiftUI
import KeychainSwift
internal import Combine

@MainActor
class AuthManager: ObservableObject {
    static let shared = AuthManager()
    private init() {}
    
    @Published var token: String? = nil // JWT token
    @Published var user_email: String = ""
    @Published var roles: [String] = [] // User's roles if logged in
    let keychain = KeychainSwift();
    
    
//    #if DEBUG
//        static func preview(withToken token: String?) -> AuthManager { // Function to have an AuthManager with an already set token
//            let manager = AuthManager()
//            manager.token = token
//            return manager
//        }
//    #endif
    
    // Sets self.token to the token in keychain if it is valid. Also sets self.roles to the user's roles. Should be called on app's startup or when logging in
    func verifyToken() async {
        LoadingManager.shared.isLoading = true;
        
        let token = self.keychain.get("token");
        
        if token == nil {
            LoadingManager.shared.isLoading = false;
            return;
        }
        
        do {
            guard let url = URL(string: "\(Config.apiUrl)/user-infos") else {
                throw UserInfosError.invalidURL;
            }
            var request = URLRequest(url: url)
            request.httpMethod = "GET"
            request.addValue("Bearer \(token ?? "")", forHTTPHeaderField: "Authorization")
            
            let (data, response) = try await URLSession.shared.data(for: request)
            
            guard let httpResponse = response as? HTTPURLResponse else {
                throw URLError(.badServerResponse)
            }
            
            guard (200...299).contains(httpResponse.statusCode) else {
                if httpResponse.statusCode == 401 {
                    self.keychain.delete("token"); // We delete the token in keychain, no need to verify another time
                    LoadingManager.shared.isLoading = false;
                    return; // If status code == 401, the token isn't valid, we return without setting self.token
                }
                throw URLError(.badServerResponse)
            }
            
            let decoded = try JSONDecoder().decode(UserInfosResponse.self, from: data)
            self.user_email = decoded.user_mail;
            self.roles = decoded.roles;
            
        } catch {
            print(error)
            LoadingManager.shared.isLoading = false;
            return; // We don't display an error message here, the user will try to log in, and have an error message if the login route doesn't work too
        }
        LoadingManager.shared.isLoading = false;
        self.token = token;
    }
    
    func logout() {
        self.keychain.delete("token"); // Remove the token from keychain
        self.token = nil;
        self.user_email = "";
        self.roles = [];
    }
}
