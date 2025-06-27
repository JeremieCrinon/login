//
//  ContentView.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 27/06/2025.
//

import SwiftUI

struct ContentView: View {
    @StateObject private var loadingManager = LoadingManager.preview()
    
    var body: some View {
        ZStack {
            Group {
                Text("Hello, World !")
            }
            
            Loading()
                .environmentObject(loadingManager)
        }
    }
}

#Preview {
    ContentView()
}
