use serde::Deserialize;

#[swift_bridge::bridge]
mod ffi {

    extern "Rust" {
        type MyIpAddress;
        fn origin(&self) -> &str;

        async fn get_my_ip_from_rust() -> MyIpAddress;
    }
}

#[derive(Deserialize)]
pub struct MyIpAddress {
    origin: String,
}

impl MyIpAddress {
    fn origin(&self) -> &str {
        &self.origin
    }
}

async fn get_my_ip_from_rust() -> MyIpAddress {
    reqwest::get("https://httpbin.org/ip")
        .await
        .unwrap()
        .json::<MyIpAddress>()
        .await
        .unwrap()
}
