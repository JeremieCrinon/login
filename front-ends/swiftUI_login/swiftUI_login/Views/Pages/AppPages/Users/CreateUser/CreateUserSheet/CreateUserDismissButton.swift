//
//  CreateUserDismissButton.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 04/07/2025.
//

import SwiftUI

struct CreateUserDismissButton: View {
    @Binding var isShowingSheet: Bool
    var body: some View {
        Button(action: {
            isShowingSheet = false
        }) {
            Image(systemName: "xmark.circle.fill")
                .symbolRenderingMode(.palette)
                .foregroundStyle(.black, .white)
                .font(.system(size: 42))
                .fontWeight(.light)
        }
        
    }
}

#Preview {
    @Previewable @State var isShowingSheet = false
    CreateUserDismissButton(isShowingSheet: $isShowingSheet)
}
