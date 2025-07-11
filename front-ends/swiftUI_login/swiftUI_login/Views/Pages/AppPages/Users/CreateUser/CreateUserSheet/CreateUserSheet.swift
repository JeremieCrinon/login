//
//  CreateUserSheet.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 04/07/2025.
//

import SwiftUI

struct CreateUserSheet: View {
    @EnvironmentObject var createUserViewModel: CreateUserViewModel
    
    var body: some View {
        VStack {
            HStack {
                CreateUserDismissButton(isShowingSheet: $createUserViewModel.opended)
                
                Spacer()
                
                Text("create_user", comment: "The title of the create user sheet.")
                    .font(.headline)
                
                Spacer()
                
                CreateUserValidateButton()
                    .environmentObject(createUserViewModel)
            }
            .padding()
            
            CreateUserForm()
                .environmentObject(createUserViewModel)
        }
        .background(Color(.systemGroupedBackground))
    }
}

#Preview {
    CreateUserSheet()
        .environmentObject(CreateUserViewModel())
}
