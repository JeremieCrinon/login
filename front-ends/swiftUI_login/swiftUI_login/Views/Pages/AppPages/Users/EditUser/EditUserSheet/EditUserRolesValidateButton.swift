//
//  EditUserRolesValidateButton.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 24/07/2025.
//

import SwiftUI

struct EditUserRolesValidateButton: View {
    @EnvironmentObject var editUserViewModel: EditUserViewModel
    @EnvironmentObject var usersViewModel: UsersViewModel
    
    var body: some View {
        Button(action: {
            Task {
                await editUserViewModel.editUserRoles()
                await usersViewModel.getUsers()
            }
        }) {
            Text("edit_user_role", comment: "The text on the button to edit the roles of another user as an admin")
        }
    }
}

#Preview {
    EditUserRolesValidateButton()
        .environmentObject(EditUserViewModel())
        .environmentObject(UsersViewModel())
}
