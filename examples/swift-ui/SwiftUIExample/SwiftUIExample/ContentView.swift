//
//  ContentView.swift
//  SwiftUIExample
//
//  Created by Frankie Nwafili on 11/27/21.
//

import SwiftUI

struct ContentView: View {
    @EnvironmentObject var reRenderTrigger: RerenderTrigger
    var rustApp: RustApp
    
    var body: some View {
        VStack {
            Spacer()
            
            HStack {
                Spacer()
                
                rustApp.render().renderButton()
                
                Spacer()
            }
            
            Spacer()
        }
        .padding()
    }
}


class SwiftUIButton {
    var text: SwiftUIText
    var action: ButtonAction
    
    init(text: SwiftUIText, action: ButtonAction) {
        self.text = text
        self.action = action
    }
    
    func renderButton () -> ButtonView {
        ButtonView(text: text, action: action)
    }
}

struct ButtonView: View {
    var text: SwiftUIText
    var action: ButtonAction
    
    init(text: SwiftUIText, action: ButtonAction) {
        self.text = text
        self.action = action
    }
    
    var body: some View {
        Button(action: {
            action.call()
        }) {
            text.text
        }
    }
}

class SwiftUIText {
    var text: Text
    
    init (text: RustStr) {
        self.text = Text(text.toString())
    }
    
    func bold() {
        text = text.bold()
    }
}


struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        let rerenderTrigger = RerenderTrigger()
        
        ContentView(rustApp: RustApp.new(rerenderTrigger))
            .environmentObject(rerenderTrigger)
    }
}

