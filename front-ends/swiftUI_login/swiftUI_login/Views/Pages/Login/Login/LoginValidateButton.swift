//
//  LoginValidateButton.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 29/06/2025.
//

import SwiftUI

struct LoginValidateButton: View {
    @EnvironmentObject var loginViewModel: LoginViewModel
    @EnvironmentObject var loadingManager: LoadingManager
    
    var disabled: Bool {
        loginViewModel.email.isEmpty || loadingManager.isLoading
    }
    
    var body: some View {
        Button(action: {
            Task {
                await loginViewModel.login()
            }
        }) {
            Text("login_button", comment: "The text on the login button")
                .padding()
                .font(.title2)
                .fontWeight(.semibold)
                .frame(maxWidth: .infinity)
        }
        .background(disabled ? Color.gray.opacity(0.4) : Color.blue)
        .foregroundStyle(disabled ? Color.gray : Color.white)
        .cornerRadius(50)
        .disabled(disabled)
        .opacity(disabled ? 0.5 : 1)
        .padding(.horizontal, 40)
        
    }
}

#Preview {
    LoginValidateButton()
        .environmentObject(LoginViewModel())
        .environmentObject(LoadingManager.shared)
}
