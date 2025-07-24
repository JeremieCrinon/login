//
//  EditUserSheet.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 23/07/2025.
//

import SwiftUI

struct EditUserSheet: View {
    @EnvironmentObject var editUserViewModel: EditUserViewModel
    
    var body: some View {
        VStack {
            HStack {
                EditUserDismissButton(isShowingSheet: $editUserViewModel.isOpened)
                
                Spacer()
                
                Text("edit_user", comment: "The title of the edit user sheet.")
                    .font(.headline)
                
                Spacer()
            }
            .padding()
            .padding(.trailing, 42) // To compensate from the absence of validate button on the right. The text isn't centered else
            
            EditUserForm()
        }
        .background(Color(.systemGroupedBackground))
    }
}

#Preview {
    EditUserSheet()
        .environmentObject(EditUserViewModel())
}
