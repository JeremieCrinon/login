//
//  EditEmailForm.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 01/07/2025.
//

import SwiftUI

struct EditEmailForm: View {
    @StateObject private var editEmailViewModel = EditEmailViewModel()
    @State private var email: String = ""
    @State private var password: String = ""
    
    var body: some View {
        Text("edit_email", comment: "The title of the form to edit the email in the account page")
            .font(.title2)
            .fontWeight(.semibold)
        
        Form {
            Group {
                if let error = editEmailViewModel.error {
                    Text(LocalizedStringKey(error.localizationKey))
                        .foregroundStyle(.red)
                }
                
                if let error = editEmailViewModel.validationError {
                    Text(LocalizedStringKey(error.localizationKey))
                        .foregroundStyle(.red)
                }
            }
            
            Group {
                TextField("email", text: $editEmailViewModel.email)
                    .autocapitalization(.none)
                    .keyboardType(.emailAddress)
                    .disableAutocorrection(true)
                
                SecureField("password", text: $editEmailViewModel.password)
                    .autocapitalization(.none)
                    .disableAutocorrection(true)
            }
            
            Group {
                EditEmailValidateButton()
                    .environmentObject(editEmailViewModel)
            }
        }
    }
}

#Preview {
    EditEmailForm()
}
