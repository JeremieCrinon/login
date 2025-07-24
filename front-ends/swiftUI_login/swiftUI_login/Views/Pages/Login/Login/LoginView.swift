//
//  LoginView.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 29/06/2025.
//

import SwiftUI

struct LoginView: View {
    @StateObject private var loginViewModel = LoginViewModel()
    
    var body: some View {
        VStack {
            ZStack {
                Text("login", comment: "The title of the login page")
                    .font(.title)
                    .fontWeight(.semibold)
                
            }
            
            
            LoginForm()
                .environmentObject(loginViewModel)
        }
    }
}

#Preview {
    LoginView()
}
