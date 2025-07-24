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
                    TabView {
                        Tab("home", systemImage: "house.fill") {
                            Text("home")
                        }
                        
                        if authManager.roles.contains("edit_users") || authManager.roles.contains("admin") {
                            Tab("users", systemImage: "person.3.sequence.fill") {
                                UsersView()
                            }
                        }
                        
                        Tab("account", systemImage: "person.crop.circle.fill") {
                            AccountView()
                        }
                        
                    }
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
