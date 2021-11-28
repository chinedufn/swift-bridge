//
//  SwiftUIExampleApp.swift
//  SwiftUIExample
//
//  Created by Frankie Nwafili on 11/27/21.
//

import SwiftUI

@main
struct SwiftUIExampleApp: App {
    var rerender = RerenderTrigger()
    
    var body: some Scene {
        let rustApp = RustApp.new(rerender)
        
        WindowGroup {
            ContentView(
                rustApp: rustApp
            )
                .environmentObject(rerender)
        }
    }
}

/// Rust uses this to make the app rerender.
class RerenderTrigger: ObservableObject {
    // Whenever this value changes the app will re-render.
    // The Rust app will incremenet this number in order to trigger
    // a re-render.
    @Published var renderCount: UInt = 0
    
    func render() {
        renderCount += 1
    }
}
