//
//  VerifyEmailView.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 30/06/2025.
//

import SwiftUI

struct VerifyEmailView: View {
    @StateObject private var verifyEmailViewModel = VerifyEmailViewModel()
    
    var body: some View {
        VStack {
            ZStack {
                Text("verify_email", comment: "The title of the verify email adress page")
                    .font(.title)
                    .fontWeight(.semibold)
                    .frame(width: 250)
                
            }
            
            VerifyEmailForm()
                .environmentObject(verifyEmailViewModel)
        }
    }
}

#Preview {
    VerifyEmailView()
}
