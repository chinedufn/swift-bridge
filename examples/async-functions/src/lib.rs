use std::collections::HashMap;

#[swift_bridge::bridge]
mod ffi {
    #[swift_bridge(swift_repr = "struct")]
    struct MyIpAddress {
        origin: String,
    }

    extern "Rust" {
        async fn get_my_ip_from_rust() -> MyIpAddress;
    }
}

// TODO: Return a `Result<MyIpAddress, SomeErrorType>`
//  Once we support returning Result from an async function.
async fn get_my_ip_from_rust() -> ffi::MyIpAddress {
    println!("Starting HTTP request from the Rust side...");

    let origin = reqwest::get("https://httpbin.org/ip")
        .await
        .unwrap()
        .json::<HashMap<String, String>>()
        .await
        .unwrap()
        .remove("origin")
        .unwrap();

    println!("HTTP request complete. Returning the value to Swift...");

    ffi::MyIpAddress { origin }
}
