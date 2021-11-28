//
//  CodegenVisualizerApp.swift
//  CodegenVisualizer
//
//  Created by Frankie Nwafili on 11/28/21.
//

import SwiftUI

@main
struct CodegenVisualizerApp: App {
    var body: some Scene {
        let generatedCodeHolder = GeneratedCodeHolder()
        let rustApp = makeRustApp(generatedCodeHolder)
        
        WindowGroup {
            ContentView(
                rustApp: rustApp
            )
                .environmentObject(generatedCodeHolder)
        }
    }
    
    func makeRustApp(_ generatedCodeHolder: GeneratedCodeHolder) -> RustApp {
        let rustApp = RustApp(generatedCodeHolder)
        rustApp.start_generated_rust_code_formatter_thread()
        
        return rustApp
    }
}
