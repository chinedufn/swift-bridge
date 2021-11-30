use crossbeam_channel::{Receiver, Sender};
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::io::Write;
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use swift_bridge_ir::SwiftBridgeModule;

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type RustApp;

        #[swift_bridge(init)]
        fn new(generated_code_holder: GeneratedCodeHolder) -> RustApp;

        fn start_generated_rust_code_formatter_thread(&self);

        // TODO: We really want `String` or `&str` here... but need to make our implementations
        //  more sound since right now (Nov 30 2021) this leads to a bug where if Swift's automatic
        //  reference counter drops the undelying Swift standard String before our function runs
        //  we end up getting a corrupted String.
        fn generate_swift_bridge_code(&self, bridge_module_source: &str);
    }

    extern "Swift" {
        type GeneratedCodeHolder;

        #[swift_bridge(swift_name = "setGeneratedRust")]
        fn set_generated_rust(&self, rust: &str);

        #[swift_bridge(swift_name = "setGeneratedSwift")]
        fn set_generated_swift(&self, swift: &str);

        #[swift_bridge(swift_name = "setGeneratedCHeader")]
        fn set_generated_c_header(&self, c: &str);

        #[swift_bridge(swift_name = "setErrorMessage")]
        fn set_error_message(&self, error: &str);
    }
}

pub struct RustApp {
    generated_code_holder: Arc<ffi::GeneratedCodeHolder>,
    format_sender: Sender<()>,
    format_receiver: Receiver<()>,
    most_recent_rust_source: Arc<Mutex<String>>,
}

unsafe impl Send for ffi::GeneratedCodeHolder {}
unsafe impl Sync for ffi::GeneratedCodeHolder {}

impl RustApp {
    pub fn new(generated_code_holder: ffi::GeneratedCodeHolder) -> Self {
        let (format_sender, format_receiver) = crossbeam_channel::unbounded();

        RustApp {
            generated_code_holder: Arc::new(generated_code_holder),
            format_sender,
            format_receiver,
            most_recent_rust_source: Default::default(),
        }
    }

    fn start_generated_rust_code_formatter_thread(&self) {
        let most_recent_rust_source = Arc::clone(&self.most_recent_rust_source);
        let receiver = self.format_receiver.clone();

        let holder = Arc::clone(&self.generated_code_holder);

        std::thread::spawn(move || {
            while let Ok(()) = receiver.recv() {
                println!("Reran rustfmt");

                let most_recent_rust_source = most_recent_rust_source.lock().unwrap();
                let generated_rust = generate_code(&most_recent_rust_source);
                if generated_rust.is_err() {
                    continue;
                }

                let generated_rust = generated_rust.expect("Generated Rust");

                let mut command = Command::new("bash")
                    .args(&["-c", "$HOME/.cargo/bin/rustfmt"])
                    .arg("--edition=2018")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .unwrap();

                let mut stdin = command.stdin.take().unwrap();
                std::thread::spawn(move || {
                    stdin
                        .write_all(generated_rust.rust.as_bytes())
                        .expect("Failed to write to stdin");
                });

                let output = command.wait_with_output().expect("Rustfmt Output");
                let err = String::from_utf8(output.stderr.to_vec()).expect("Rustfmt stderr");
                let generated = String::from_utf8(output.stdout.to_vec()).expect("Rustfmt stdout");

                if err.len() > 0 {
                    dbg!(&err);
                }

                holder.set_generated_rust(&generated);
            }
        });
    }

    pub fn generate_swift_bridge_code(&self, bridge_module_source: &str) {
        let mut previous = self.most_recent_rust_source.lock().unwrap();

        let previous_tokens = TokenStream::from_str(&previous).ok().map(|t| t.to_string());
        let new_tokens = TokenStream::from_str(&bridge_module_source)
            .ok()
            .map(|t| t.to_string());

        if previous_tokens == new_tokens {
            return;
        }

        let holder = &self.generated_code_holder;

        let generated = generate_code(&bridge_module_source);
        if let Ok(generated) = generated {
            holder.set_generated_swift(&generated.swift);
            holder.set_generated_c_header(&generated.c_header);
            holder.set_error_message("");

            self.format_sender.send(()).unwrap();
        } else {
            let err = generated.err().unwrap();
            holder.set_error_message(&err);
        }

        *previous = bridge_module_source.to_string();
    }
}

struct GeneratedCode {
    rust: String,
    swift: String,
    c_header: String,
}

fn generate_code(bridge_module_source: &str) -> Result<GeneratedCode, String> {
    let token_stream = TokenStream::from_str(bridge_module_source).map_err(|e| e.to_string())?;

    let generated: SwiftBridgeModule = syn::parse2(token_stream).map_err(|e| e.to_string())?;

    let rust = generated.to_token_stream().to_string();

    Ok(GeneratedCode {
        rust,
        swift: generated.generate_swift(),
        c_header: generated.generate_c_header(),
    })
}
