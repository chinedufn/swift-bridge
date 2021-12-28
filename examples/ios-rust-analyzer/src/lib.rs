#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type RustApp;

        #[swift_bridge(init)]
        fn new() -> RustApp;

        fn generate_html(&self, rust_code: &str) -> String;
    }
}

pub struct RustApp {}

impl RustApp {
    fn new() -> Self {
        RustApp {}
    }

    fn generate_html(&self, rust_code: &str) -> String {
        let (analysis, file_id) = ide::Analysis::from_single_file(rust_code.to_string());

        analysis
            .highlight_as_html(file_id, true)
            .unwrap_or("Error".to_string())
    }
}
