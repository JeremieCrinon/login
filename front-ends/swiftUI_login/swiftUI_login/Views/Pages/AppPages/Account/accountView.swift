//
//  AccountView.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 01/07/2025.
//

import SwiftUI

struct AccountView: View {
    var authManager = AuthManager.shared
    var body: some View {
        Text("account")
            .font(.title)
            .fontWeight(.semibold)
        
        Form {
            EditEmailForm()
        
            Spacer()
            
            EditPasswordForm()
            
            Spacer()
            
            Logout()
                .environmentObject(authManager)
        }
            
        
    }
    
}

#Preview {
    AccountView()
}
