
#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        //fn fn1(input: Result<i8,i8>) -> Result<i8,i8>;
        //fn fn2(_input: Result<i8,i8>);
        fn fn3(_input: Option<Option<i8>>) -> Option<Option<i8>>;
    }
}

fn fn1(input: Result<i8,i8>) -> Result<i8,i8>{input}
fn fn2(_input: Result<i8,i8>){}
fn fn3(input: Option<i8>) -> Option<i8>{input}

fn main() {}
