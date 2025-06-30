//
//  NewAccountForm.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 30/06/2025.
//

import SwiftUI

struct NewAccountForm: View {
    @EnvironmentObject var newAccountViewModel: NewAccountViewModel
    
    var body: some View {
        Form {
            Group {
                if let error = newAccountViewModel.error {
                    Text(LocalizedStringKey(error.localizationKey))
                        .foregroundStyle(.red)
                }
                
                if let error = newAccountViewModel.validationError {
                    Text(LocalizedStringKey(error.localizationKey))
                        .foregroundStyle(.red)
                }
            }
            Group {
                TextField("email", text: $newAccountViewModel.email)
                    .autocapitalization(.none)
                    .keyboardType(.emailAddress)
                    .disableAutocorrection(true)
                
                HStack {
                    SecureField("password", text: $newAccountViewModel.password)
                        .autocapitalization(.none)
                        .disableAutocorrection(true)
                    
                    SecureField("password_confirm", text: $newAccountViewModel.confirmPassword)
                        .autocapitalization(.none)
                        .disableAutocorrection(true)
                }
                
            }
            
        }
    }
}

#Preview {
    NewAccountForm()
        .environmentObject(NewAccountViewModel())
}
