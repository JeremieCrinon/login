//
//  EditEmailValidateButton.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 01/07/2025.
//

import SwiftUI

struct EditEmailValidateButton: View {
    @EnvironmentObject var editEmailViewModel: EditEmailViewModel
    
    var body: some View {
        Button(action: {
            Task {
                await editEmailViewModel.editEmail()
            }
        }) {
            Text("edit_email_validate", comment: "The text on the button to validate the email change on the account page")
        }
    }
}

#Preview {
    EditEmailValidateButton()
}
