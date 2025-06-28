//
//  ContentView.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 27/06/2025.
//

import SwiftUI

struct ContentView: View {
    @StateObject private var loadingManager = LoadingManager.shared
    @StateObject private var authManager = AuthManager.shared
    
    var body: some View {
        ZStack {
            Group {
                if authManager.token == nil {
                    Text("Not logged in")
                } else if authManager.roles.contains("new_account") {
                    Text("New account")
                } else if authManager.roles.contains("unverified_email") {
                    Text("Unverified email")
                } else {
                    Text("Logged in")
                }
            }
            
            Loading()
                .environmentObject(loadingManager)
        }
        .task {
            await authManager.verifyToken() // We call this function when the app is loaded to know if the user has a token in the keychain, and if this token is valid, and if it is valid, what roles the user has.
        }
    }
}

#Preview {
    ContentView()
}
