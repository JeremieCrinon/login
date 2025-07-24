//
//  EditUserToggleButton.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 23/07/2025.
//

import SwiftUI

struct EditUserToggleButton: View {
    var userId: Int
    @EnvironmentObject var editUserViewModel: EditUserViewModel
    var body: some View {
        Button(action: {
            Task {
                await editUserViewModel.getUser(userId: userId)
            }
        }) {
            Image(systemName: "info")
        }
    }
}

#Preview {
    EditUserToggleButton(userId: 1)
        .environmentObject(EditUserViewModel())
}
