import SwiftUI

@main
struct IosRustAnalyzerApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(RustAppWrapper(rust: RustApp()))
        }
    }
}

class RustAppWrapper: ObservableObject {
    var rust: RustApp
    
    init (rust: RustApp) {
        self.rust = rust
    }
}

