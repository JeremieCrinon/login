//
//  EditPasswordValidateButton.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 02/07/2025.
//

import SwiftUI

struct EditPasswordValidateButton: View {
    @EnvironmentObject var editPasswordViewModel: EditPasswordViewModel
    
    var body: some View {
        Button(action: {
            Task {
                await editPasswordViewModel.EditPassword()
            }
        }) {
            Text("edit_password_validate", comment: "The text on the button to validate the password change on the account page")
        }
    }
}

#Preview {
    EditPasswordValidateButton()
        .environmentObject(EditEmailViewModel())
}
