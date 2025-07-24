//
//  CreateUserToggleButton.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 04/07/2025.
//

import SwiftUI

struct CreateUserToggleButton: View {
    @Binding var isShowingSheet: Bool
    
    var body: some View {
        
        Button(action: {
            isShowingSheet.toggle()
        }) {
            Image(systemName: "plus.circle.fill")
                .font(.system(size: 42))
                .fontWeight(.light)
        }
    }
}

#Preview {
    @Previewable @State var isShowingSheet = false
    CreateUserToggleButton(isShowingSheet: $isShowingSheet)
}
