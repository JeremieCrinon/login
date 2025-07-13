//
//  UsersList.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 13/07/2025.
//

import SwiftUI

struct UsersList: View {
    @StateObject private var usersViewModel: UsersViewModel = UsersViewModel()
    var body: some View {
        List {
            ForEach(usersViewModel.users, id: \.self) { user in
                UserRow(user: user)
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
}
