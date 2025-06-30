//
//  LoginForm.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 30/06/2025.
//

import SwiftUI

struct LoginForm: View {
    @EnvironmentObject var loginViewModel: LoginViewModel
    
    var body: some View {
        Form {
            Group {
                if let error = loginViewModel.error {
                    Text(LocalizedStringKey(error.localizationKey))
                        .foregroundStyle(.red)
                }
            }
            Group {
                TextField("email", text: $loginViewModel.email)
                    .autocapitalization(.none)
                    .keyboardType(.emailAddress)
                    .disableAutocorrection(true)
                
                SecureField("password", text: $loginViewModel.password)
                    .autocapitalization(.none)
                    .disableAutocorrection(true)
            }
            
            Group {
                LoginValidateButton()
                    .environmentObject(loginViewModel)
                    .environmentObject(LoadingManager.shared)
                    .padding()
            }
        }
    }
}

#Preview {
    LoginForm()
        .environmentObject(LoginViewModel())
}
