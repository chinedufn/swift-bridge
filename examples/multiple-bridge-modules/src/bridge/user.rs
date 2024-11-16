#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type User;

        #[swift_bridge(get(&name))]
        fn name(&self) -> &str;

        fn make_user() -> User;
    }
}

struct User {
    #[allow(dead_code)]
    name: String,
}

fn make_user() -> User {
    User {
        name: "Bob".to_string(),
    }
}
