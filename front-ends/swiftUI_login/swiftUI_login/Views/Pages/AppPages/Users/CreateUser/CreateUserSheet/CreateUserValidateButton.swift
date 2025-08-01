//
//  CreateUserValidateButton.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 04/07/2025.
//

import SwiftUI

struct CreateUserValidateButton: View {
    @EnvironmentObject var createUserViewModel: CreateUserViewModel
    @EnvironmentObject var usersViewModel: UsersViewModel
    
    var body: some View {
        Button(action: {
            Task {
                await createUserViewModel.createUser()
                await usersViewModel.getUsers()
            }
        }) {
            Image(systemName: "checkmark.circle.fill")
                .font(.system(size: 42))
                .fontWeight(.light)
        }
    }
}

#Preview {
    CreateUserValidateButton()
        .environmentObject(CreateUserViewModel())
        .environmentObject(UsersViewModel())
}
