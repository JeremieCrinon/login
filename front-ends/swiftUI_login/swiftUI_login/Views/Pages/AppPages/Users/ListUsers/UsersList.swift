//
//  UsersList.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 13/07/2025.
//

import SwiftUI

struct UsersList: View {
    @EnvironmentObject var usersViewModel: UsersViewModel
    @StateObject private var deleteUserViewModel: DeleteUserViewModel = DeleteUserViewModel()
    @StateObject private var editUserViewModel: EditUserViewModel = EditUserViewModel()
    
    var body: some View {
        if let error = deleteUserViewModel.error {
            Text(LocalizedStringKey(error.localizationKey))
                .foregroundStyle(.red)
        }
        
        List {
            ForEach(usersViewModel.users, id: \.self) { user in
                UserRow(user: user)
                    .swipeActions(edge: .trailing) {
                        Button(role: user.id == 1 || user.email == AuthManager.shared.user_email ? .none : .destructive, action: {
                            Task {
                                if user.id != 1 {
                                    await deleteUserViewModel.deleteUser(user: user)
                                    
                                    await usersViewModel.getUsers()
                                }
                            }
                        }) {
                            Image(systemName: "trash")
                        }
                        
                        EditUserToggleButton(userId: user.id)
                            .environmentObject(editUserViewModel)
                    }
            }
        }
        .task {
            await usersViewModel.getUsers()
        }
        .background(Color(.systemGroupedBackground))
        .sheet(isPresented: $editUserViewModel.isOpened, onDismiss: {editUserViewModel.isOpened = false}) {
            EditUserSheet()
                .environmentObject(editUserViewModel)
        }
    }
}

#Preview {
    UsersList()
        .environmentObject(UsersViewModel())
}
