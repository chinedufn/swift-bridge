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
    out_dir: impl AsRef<Path>,
) {
    let out_dir = out_dir.as_ref();

    for rust_file in rust_source_files.into_iter() {
        let rust_file: &Path = rust_file.as_ref();

        let file = std::fs::read_to_string(rust_file).unwrap();
        let generated = parse_file(&file).unwrap();

        // TODO: .. Ok... what to do with this generated C header and Swift..?
        // We need to write them to disk somewhere..
        // Then we #include the C header from Swift
        // And add the Swift file to the project
        // For now let's just write them to a hard coded place on disk and see everything work..
        // then we can use variables instead of hard coded paths.

        let c_header_file_name = rust_file.with_extension("h");
        let c_header_file_name = c_header_file_name.file_name().unwrap();
        let c_header_file = out_dir.join(c_header_file_name);
        std::fs::write(&c_header_file, generated.c_header).unwrap();

        let swift_file_name = rust_file.with_extension("swift");
        let swift_file_name = swift_file_name.file_name().unwrap();
        let swift_file = out_dir.join(swift_file_name);
        std::fs::write(&swift_file, generated.swift).unwrap();
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
                if module
                    .attrs
                    .iter()
                    .any(|a| a.path.to_token_stream().to_string() == "swift_bridge :: bridge")
                {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        let generated = parse_file(MOCK_FILE).unwrap();
        dbg!(&generated);
        todo!("Delete this test.. just a scratchpad..")
    }

    const MOCK_FILE: &'static str = r#"
#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type Foo;
    }
} 
    "#;
}
