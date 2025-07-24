//
//  ContentView+Message.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 30/06/2025.
//

import SwiftUI

extension View {
    func globalAlert(_ messagesManager: MessagesManager) -> some View {
        self.alert(isPresented: Binding<Bool>(
            get: { messagesManager.title != nil },
            set: { _ in
                messagesManager.title = nil
                messagesManager.message = nil
            }
        )) {
            Alert(
                title: Text(LocalizedStringKey(messagesManager.title!.localizationKey)),
                message: Text(LocalizedStringKey(messagesManager.message!.localizationKey)),
                dismissButton: .default(Text("OK"))
            )
        }
    }
}
