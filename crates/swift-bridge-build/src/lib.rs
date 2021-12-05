//! Parse Rust source files for #\[swift_bridge::bridge\] modules and then generate the
//! corresponding C header files and Swift files.

#![deny(missing_docs)]

use std::path::Path;
use swift_bridge_ir::SwiftBridgeModule;
use syn::__private::ToTokens;
use syn::{File, Item};

/// Parse rust sources files for `#\[swift_bridge::bridge\]` headers and generate the corresponding
/// Swift files.
pub fn parse_bridges(
    rust_source_files: impl IntoIterator<Item = impl AsRef<Path>>,
) -> GeneratedCode {
    let mut generated_code = GeneratedCode::new();

    for rust_file in rust_source_files.into_iter() {
        let rust_file: &Path = rust_file.as_ref();

        let file = std::fs::read_to_string(rust_file).unwrap();
        let gen = parse_file(&file).unwrap();

        generated_code.generated.push(gen);
    }

    generated_code
}

/// Generated Swift files and C headers.
pub struct GeneratedCode {
    generated: Vec<GeneratedFromSwiftBridgeModule>,
}

impl GeneratedCode {
    fn new() -> Self {
        GeneratedCode { generated: vec![] }
    }
}

impl GeneratedCode {
    /// Write all of the generated Swift to a single Swift file and all of the generated C headers
    /// to a single header file.
    pub fn write_all_concatenated(
        &self,
        swift_bridge_out_dir: impl AsRef<Path>,
        package_name: &str,
    ) {
        let swift_bridge_out_dir = swift_bridge_out_dir.as_ref();

        let mut concatenated_swift = "".to_string();
        let mut concatenated_c = "".to_string();

        for gen in &self.generated {
            concatenated_swift += &gen.swift;
            concatenated_c += &gen.c_header;
        }

        let out = swift_bridge_out_dir.join(&package_name);
        match std::fs::create_dir_all(&out) {
            Ok(_) => {}
            Err(_) => {}
        };

        std::fs::write(out.join(format!("{}.h", package_name)), concatenated_c).unwrap();
        std::fs::write(
            out.join(format!("{}.swift", package_name)),
            concatenated_swift,
        )
        .unwrap();
    }

    /// Concatenate all of the generated Swift code into one file.
    pub fn concat_swift(&self) -> String {
        let mut swift = "".to_string();

        for gen in &self.generated {
            swift += &gen.swift;
        }

        swift
    }

    /// Concatenate all of the generated C code into one file.
    pub fn concat_c(&self) -> String {
        let mut c_header = "".to_string();

        for gen in &self.generated {
            c_header += &gen.c_header;
        }

        c_header
    }
}

fn parse_file(file: &str) -> syn::Result<GeneratedFromSwiftBridgeModule> {
    let file: File = syn::parse_str(file)?;

    let mut generated = GeneratedFromSwiftBridgeModule {
        c_header: "".to_string(),
        swift: "".to_string(),
    };

    for item in file.items {
        match item {
            Item::Mod(module) => {
                // TODO: Move this check into the `impl Parse for SwiftBridgeModule`.. Modify our
                //  tests in swift-bridge-ir to annotate modules with `#[swift_bridge::bridge]`
                if module.attrs.iter().any(|a| {
                    let attrib = a.path.to_token_stream().to_string();
                    attrib == "swift_bridge :: bridge" || attrib == "swift_bridge_macro :: bridge"
                }) {
                    let module: SwiftBridgeModule = syn::parse2(module.to_token_stream())?;

                    let c_header = module.generate_c_header();
                    generated.c_header += &c_header;
                    generated.c_header += "\n\n";

                    let swift = module.generate_swift();
                    generated.swift += &swift;
                    generated.swift += "\n\n";
                }
            }
            _ => {}
        }
    }

    Ok(generated)
}

#[derive(Debug)]
struct GeneratedFromSwiftBridgeModule {
    c_header: String,
    swift: String,
}
