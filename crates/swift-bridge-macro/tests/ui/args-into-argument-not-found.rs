//! # To Run
//! cargo test -p swift-bridge-macro -- ui trybuild=args-into-argument-not-found.rs

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        #[swift_bridge(args_into = (arg, arg_typo))]
        fn some_function(arg: u8);
    }
}

fn some_function(arg: u8){

}

fn main() {

}