//
//  EditUserDismissButton.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 23/07/2025.
//

import SwiftUI

struct EditUserDismissButton: View {
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
    EditUserDismissButton(isShowingSheet: $isShowingSheet)
}
