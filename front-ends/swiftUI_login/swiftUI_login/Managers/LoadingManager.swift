//
//  LoadingManager.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 27/06/2025.
//

import Foundation
import SwiftUI
internal import Combine

class LoadingManager: ObservableObject {
    static let shared = LoadingManager()
    
    @Published var isLoading: Bool = false
    
    
    private init() {}
    
    #if DEBUG
    static func preview() -> LoadingManager { // This function ables previews to call that instead of shared to have the app in a loading state
        let manager = LoadingManager()
        manager.isLoading = true
        return manager
    }
    #endif
}
