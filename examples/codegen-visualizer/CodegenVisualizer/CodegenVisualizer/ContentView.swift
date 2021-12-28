//
//  ContentView.swift
//  CodegenVisualizer
//
//  Created by Frankie Nwafili on 11/28/21.
//

import SwiftUI
import Combine

import CodeEditor

struct ContentView: View {
    var rustApp: RustApp
    @EnvironmentObject var generatedCodeHolder: GeneratedCodeHolder
    
    @State private var rustSource = """
    #[swift_bridge::bridge]
    mod ffi {
        extern "Rust" {
        }
    
        extern "Rust" {
        }
    
        extern "Swift" {
        }
    
        extern "Swift" {
        }
    }
    """
    
    var generatedCHeader = "typedef struct Foo Foo;"
    var generatedSwift = "func main () {}"
    var generatedRust = "let foo = 0"
    
    let swiftSectionHeaderBackground = Color.gray
    let cSectionHeaderBackground = Color(red: 0.34, green: 0.35, blue: 0.36, opacity: 1.0)
    let rustSectionHeaderBackground = Color(red: 0.24, green: 0.25, blue: 0.26, opacity: 1.0)
    
    
    var body: some View {
        
        VStack {
            if generatedCodeHolder.errorMessage.count > 0 {
                Text("\(generatedCodeHolder.errorMessage)")
            } else {
                Text("")
            }
            
            HStack {
                VStack {
                    CodeEditor(
                        source: $rustSource,
                        language: .rust
                    )
                        .onReceive(Just(rustSource), perform: {source in
                            rustApp.generate_swift_bridge_code(source)
                        })
                    
                    GeneratedCodeView(
                        header: "Generated C Header",
                        headerBackgroundColor: cSectionHeaderBackground,
                        editor: CodeEditor(
                            source: generatedCodeHolder.generatedC,
                            language: .c,
                            theme: .ocean
                        )
                    )
                }
                
                VStack {
                    GeneratedCodeView (
                        header: "Generated Rust Code",
                        headerBackgroundColor: rustSectionHeaderBackground,
                        editor: CodeEditor(
                            source: generatedCodeHolder.generatedRust,
                            language: .rust,
                            theme: .pojoaque
                        ))
                    
                    GeneratedCodeView (
                        header: "Generated Swift Code",
                        headerBackgroundColor: swiftSectionHeaderBackground,
                        editor:
                            CodeEditor(
                                source: generatedCodeHolder.generatedSwift,
                                language: .swift,
                                theme: .agate
                            )
                    )
                }
            }
        }
    }
}


struct GeneratedCodeView: View {
    var header: String
    var headerBackgroundColor: Color
    var editor: CodeEditor
    
    var body: some View {
        VStack {
            HStack {
                Text(header)
                    .font(.title)
                    .padding(0)
                Spacer()
            }
            .background(headerBackgroundColor)
            editor
        }
    }
}

class GeneratedCodeHolder: ObservableObject {
    @Published var generatedRust = ""
    @Published var generatedSwift = ""
    @Published var generatedC = ""
    @Published var errorMessage = ""
    
    init() { }
    
    func setGeneratedRust (rust: RustStr) {
        let code = rust.toString()
        DispatchQueue.main.async {
            self.generatedRust = code
        }
    }
    
    func setGeneratedSwift (swift: RustStr) {
        let swiftCode = swift.toString()
        DispatchQueue.main.async {
            self.generatedSwift = swiftCode
        }
    }
    
    func setGeneratedCHeader (c: RustStr) {
        let cHeader = c.toString()
        DispatchQueue.main.async {
            self.generatedC = cHeader
        }
    }
    
    func setErrorMessage (error: RustStr) {
        let err = error.toString()
        DispatchQueue.main.async {
            self.errorMessage = err
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        let generatedCodeHolder = GeneratedCodeHolder()
        let rustApp = RustApp(generatedCodeHolder)
        
        ContentView(
            rustApp: rustApp
        )
            .environmentObject(GeneratedCodeHolder())
    }
}
