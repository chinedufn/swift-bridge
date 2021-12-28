import SwiftUI
import WebKit
import Combine

struct ContentView: View {
    @EnvironmentObject var rustApp: RustAppWrapper
    
    @State private var rustSource = initialSource
    @State private var rustHtml = ""
    
    var body: some View {
        VStack {
            TextEditor(text: $rustSource)
                .font(.caption)
                .onReceive(Just(rustSource), perform: {sourceCode in
                    let html = rustApp.rust.generate_html(sourceCode).toString()
                    rustHtml = html
                })
            
            WebView(text: $rustHtml)
                .frame(minWidth: 0, maxWidth: .infinity, minHeight: 0, maxHeight: .infinity)
            
        }
    }
}

struct WebView: UIViewRepresentable {
    @Binding var text: String
    
    func makeUIView(context: Context) -> WKWebView {
        return WKWebView()
    }
    
    func updateUIView(_ uiView: WKWebView, context: Context) {
        uiView.loadHTMLString(text, baseURL: nil)
    }
}

let initialSource = """

fn main () {
    let stack: Stack<u8> = Stack::default();
    
    for val in 0..100 {
        stack.push(val);
    }
}

#[derive(Default)]
struct Stack<T>(Vec<T>);

impl<T> Stack<T> {
    fn push(&mut self, val: T) {
        self.0.push(val);
    }

    fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }
}

"""
 

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
            .environmentObject(RustAppWrapper(rust: RustApp()))
    }
}

