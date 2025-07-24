//
//  VerifyEmailForm.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 01/07/2025.
//

import SwiftUI

struct VerifyEmailForm: View {
    @EnvironmentObject var verifyEmailViewModel: VerifyEmailViewModel
    var body: some View {
        Form {
            Group {
                if let error = verifyEmailViewModel.error {
                    Text(LocalizedStringKey(error.localizationKey))
                        .foregroundStyle(.red)
                }
            }
            Group {
                TextField("code", text: $verifyEmailViewModel.code)
                    .autocapitalization(.none)
                    .disableAutocorrection(true)
                    .onChange(of: verifyEmailViewModel.code) { _, newValue in
                        if newValue.count >= 7 {
                            Task {
                                await verifyEmailViewModel.verifyEmail()
                            }
                        }
                    }
            }
            
        }
    }
}

#Preview {
    VerifyEmailForm()
        .environmentObject(VerifyEmailViewModel())
}
