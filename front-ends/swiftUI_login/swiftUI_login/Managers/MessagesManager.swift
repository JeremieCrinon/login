//
//  MessagesManager.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 30/06/2025.
//

import Foundation
import SwiftUI
internal import Combine

class MessagesManager: ObservableObject {
    static let shared = MessagesManager()
    
    @Published var title: MessageTitle? = nil
    @Published var message: MessageDesc? = nil
    
    private init() {}
    
    func setMessage (title: MessageTitle, message: MessageDesc) {
        self.title = title
        self.message = message
        
        DispatchQueue.main.asyncAfter(deadline: .now() + 10) {
            // If another title as been set meanwhile, we don't want to delete it
            if self.title == title {
                self.title = nil
            }
        }
    }
}
