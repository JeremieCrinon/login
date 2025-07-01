//
//  ContentView.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 27/06/2025.
//

import SwiftUI

struct ContentView: View {
    @StateObject private var loadingManager = LoadingManager.shared
    @StateObject private var messagesManager = MessagesManager.shared
    @StateObject private var authManager = AuthManager.shared
    
    var body: some View {
        ZStack {
            Group {
                if authManager.token == nil {
                    LoginView()
                } else if authManager.roles.contains("new_account") {
                    NewAccountView()
                } else if authManager.roles.contains("unverified_email") {
                    VerifyEmailView()
                } else {
                    Logout()
                        .environmentObject(authManager)
                }
            }
            
            Loading()
                .environmentObject(loadingManager)
        }
        .task {
            await authManager.verifyToken() // We call this function when the app is loaded to know if the user has a token in the keychain, and if this token is valid, and if it is valid, what roles the user has.
        }
        .globalAlert(messagesManager)
    }
}

#Preview {
    ContentView()
}
