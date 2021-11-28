use super::RustApp;

pub(super) use self::ffi::{RerenderTrigger, SwiftUIButton, SwiftUIText};

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type RustApp;

        #[swift_bridge(associated_to = RustApp)]
        // TODO: This should be &RerenderTrigger, but as of Nov 28 that isn't supported yet.
        fn new(render_trigger: RerenderTrigger) -> RustApp;

        fn render(&self) -> SwiftUIButton;
    }

    extern "Rust" {
        type ButtonAction;

        fn call(&mut self);
    }

    extern "Swift" {
        type SwiftUIButton;

        #[swift_bridge(init)]
        fn new(text: SwiftUIText, action: ButtonAction) -> SwiftUIButton;
    }

    extern "Swift" {
        type SwiftUIText;

        #[swift_bridge(init)]
        fn new(text: &str) -> SwiftUIText;

        fn bold(&mut self);
    }

    extern "Swift" {
        type RerenderTrigger;

        fn render(&self);
    }
}

pub struct ButtonAction(pub Box<dyn FnMut()>);

impl ButtonAction {
    fn call(&mut self) {
        (self.0)()
    }
}
