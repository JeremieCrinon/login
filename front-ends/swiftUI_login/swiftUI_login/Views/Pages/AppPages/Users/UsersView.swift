//
//  UsersView.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 02/07/2025.
//

import SwiftUI

struct UsersView: View {
    @StateObject private var usersViewModel: UsersViewModel = UsersViewModel()
    
    var body: some View {
        ZStack {
            Text("users")
                .font(.title)
                .fontWeight(.semibold)
            
            HStack {
                Spacer()
                CreateUser()
                    .padding(25)
                    .environmentObject(usersViewModel)
            }
            
        }
        
        UsersList()
            .environmentObject(usersViewModel)
        
    }
}

#Preview {
    UsersView()
}
