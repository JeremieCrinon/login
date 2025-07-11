//
//  CreateUser.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 02/07/2025.
//

import SwiftUI

struct CreateUser: View {
    @StateObject private var createUserViewModel: CreateUserViewModel = CreateUserViewModel()
    
    
    var body: some View {
        CreateUserToggleButton(isShowingSheet: $createUserViewModel.opended)
            .sheet(isPresented: $createUserViewModel.opended, onDismiss: {createUserViewModel.opended = false}) {
                CreateUserSheet()
                    .environmentObject(createUserViewModel)
            }
    }
}

#Preview {
    CreateUser()
}
