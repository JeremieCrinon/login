//
//  NewAccountValidateButton.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 30/06/2025.
//

import SwiftUI

struct NewAccountValidateButton: View {
    @EnvironmentObject var newAccountViewModel: NewAccountViewModel
    
    var body: some View {
        Button(action: {
            Task {
                await newAccountViewModel.modifyNewAccount();
            }
        }) {
            Image(systemName: "checkmark")
                .imageScale(.large)
                .padding(10)
        }
        .background(Color.blue)
        .foregroundStyle(.white)
        .cornerRadius(50)
        .scaleEffect(1)
        
    }
}

#Preview {
    NewAccountValidateButton()
}
