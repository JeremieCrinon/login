//
//  NewAccountValidateButton.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 30/06/2025.
//

import SwiftUI

struct NewAccountValidateButton: View {
    @EnvironmentObject var newAccountViewModel: NewAccountViewModel
    @EnvironmentObject var loadingManager: LoadingManager
    
    var disabled: Bool {
        newAccountViewModel.password.isEmpty || loadingManager.isLoading
    }
    
    var body: some View {
        Button(action: {
            Task {
                await newAccountViewModel.modifyNewAccount()
            }
        }) {
            Text("new_account_button", comment: "The text on the new account validate form button")
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
    NewAccountValidateButton()
        .environmentObject(LoginViewModel())
        .environmentObject(AuthManager.shared)
}
