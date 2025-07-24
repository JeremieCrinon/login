//
//  Loading.swift
//  swiftUI_login
//
//  Created by Jeremie Crinon on 27/06/2025.
//

import SwiftUI

struct Loading: View {
    @EnvironmentObject var loadingManager: LoadingManager
    
    var body: some View {
        if loadingManager.isLoading {
            HStack {
                ProgressView()
                    .scaleEffect(2)
                    .padding()
                
                Text("loading", comment: "The text in the loading view next to the ProgressView")
                    .font(.title)
                    .padding()
            }
            .font(.title)
            .padding()
            .glassEffect()
            .ignoresSafeArea()
            .allowsHitTesting(true)
            .frame(maxWidth: .infinity, maxHeight: .infinity)
            .contentShape(Rectangle())
                
        }
    }
}

#Preview {
    ZStack { // We put a Zstack here to have a similar displaying as in the complete app
        #if DEBUG // We add this condition, even tough it should always be true, to really make sure we display this only in debug mode, as the preview function of loading manager is only avaible in debug mode
            Loading()
                .environmentObject(LoadingManager.preview())
        #endif
    }
    
}
