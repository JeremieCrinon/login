//
//  EditUserEmailValidateButton.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 24/07/2025.
//

import SwiftUI

struct EditUserEmailValidateButton: View {
    @EnvironmentObject var editUserViewModel: EditUserViewModel
    @EnvironmentObject var usersViewModel: UsersViewModel
    
    var body: some View {
        Button(action: {
            Task {
                await editUserViewModel.editUserEmail()
                await usersViewModel.getUsers()
            }
        }) {
            Text("edit_user_email", comment: "The text on the button to edit the email address of another user as an admin")
        }
    }
}

#Preview {
    EditUserEmailValidateButton()
        .environmentObject(EditUserViewModel())
        .environmentObject(UsersViewModel())
}
