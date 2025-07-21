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
                            Text("delete")
                        }
                        
                        Button(action: {
                            Task {
                                print("info")
                                print(user)
                            }
                        }) {
                            Text("info")
                        }
                    }
            }
        }
        .task {
            await usersViewModel.getUsers()
        }
        .background(Color(.systemGroupedBackground))
    }
}

#Preview {
    UsersList()
        .environmentObject(UsersViewModel())
}
