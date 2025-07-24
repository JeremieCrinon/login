//
//  NewAccountView.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 30/06/2025.
//

import SwiftUI

struct NewAccountView: View {
    @StateObject private var newAccountViewModel = NewAccountViewModel()
    var body: some View {
        VStack {
            ZStack {
                Text("modify_new_account", comment: "The title of the modify new account page.")
                    .font(.title)
                    .fontWeight(.semibold)
                    .frame(width: 250)
                
            }
            
            NewAccountForm()
                .environmentObject(newAccountViewModel)
        }
    }
}

#Preview {
    NewAccountView()
}
