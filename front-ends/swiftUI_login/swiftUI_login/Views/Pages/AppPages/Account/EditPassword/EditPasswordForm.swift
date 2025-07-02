//
//  EditPasswordForm.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 02/07/2025.
//

import SwiftUI

struct EditPasswordForm: View {
    @StateObject private var editPasswordViewModel: EditPasswordViewModel = EditPasswordViewModel()
    @State var old_password: String = ""
    @State var new_password: String = ""
    @State var confirm_password: String = ""
    
    var body: some View {
        Text("edit_password", comment: "The title of the form to edit the password in the account page")
            .font(.title2)
            .fontWeight(.semibold)
        
        Group {
            if let error = editPasswordViewModel.error {
                Text(LocalizedStringKey(error.localizationKey))
                    .foregroundStyle(.red)
            }
            
            if let error = editPasswordViewModel.validationError {
                Text(LocalizedStringKey(error.localizationKey))
                    .foregroundStyle(.red)
            }
        }
        
        Group {
            SecureField("current_password", text: $editPasswordViewModel.current_password)
                .autocapitalization(.none)
                .disableAutocorrection(true)
            
            SecureField("new_password", text: $editPasswordViewModel.new_password)
                .autocapitalization(.none)
                .disableAutocorrection(true)
            
            SecureField("confirm_password", text: $editPasswordViewModel.confirm_password)
                .autocapitalization(.none)
                .disableAutocorrection(true)
        }
        
        Group {
            EditPasswordValidateButton()
                .environmentObject(editPasswordViewModel)
        }
        
    }
}

#Preview {
    EditPasswordForm()
}
