//
//  Logout.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 29/06/2025.
//

import SwiftUI

struct Logout: View {
    @EnvironmentObject var authManager: AuthManager
    var body: some View {
        Button(action: {
            Task {
                authManager.logout()
            }
        }) {
            Text("logout", comment: "The text on the logout button") // Temporary, I need a logout button for testing, but I don't know how it would look like for now
        }
    }
}

#Preview {
    #if DEBUG
        Logout()
            .environmentObject(AuthManager.preview(withToken: "Test"))
    #endif
}
