//
//  LoginValidateButton.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 29/06/2025.
//

import SwiftUI

struct LoginValidateButton: View {
    @EnvironmentObject var loginViewModel: LoginViewModel
    
    var body: some View {
        Button(action: {
            Task {
                await loginViewModel.login();
            }
        }) {
            Image(systemName: "checkmark")
                .imageScale(.large)
                .padding(10)
        }
        .background(Color.blue)
        .foregroundStyle(.white)
        .cornerRadius(50)
        .scaleEffect(1)
        
    }
}

#Preview {
    LoginValidateButton()
}
