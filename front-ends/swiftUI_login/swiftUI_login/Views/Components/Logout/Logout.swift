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
            Text("logout", comment: "The text on the logout button")
                .padding()
                .font(.title2)
                .fontWeight(.semibold)
                .frame(maxWidth: .infinity)
        }
        .foregroundStyle(Color.red)
        .background(Color.gray .opacity(0.15))
        .cornerRadius(50)
        .padding(.horizontal, 40)
        
    }
}

#Preview {
    #if DEBUG
        Logout()
            .environmentObject(AuthManager.preview(withToken: "Test"))
    #endif
}
