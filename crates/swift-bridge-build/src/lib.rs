//! Parse Rust source files for #\[swift_bridge::bridge\] modules and then generate the
//! corresponding C header files and Swift files.

#![deny(missing_docs)]

use proc_macro2::TokenStream;
use std::path::Path;
use std::str::FromStr;
use swift_bridge_ir::SwiftBridgeModule;
use syn::__private::ToTokens;
use syn::{File, Item};

fn bridges(rust_source_files: impl IntoIterator<Item = impl AsRef<Path>>) -> Build {
    for rust_file in rust_source_files.into_iter() {
        let file = std::fs::read_to_string(rust_file.as_ref()).unwrap();
        let generated = parse_file(&file).unwrap();

        // TODO: .. Ok... what to do with this generated C header and Swift..?
        // We need to write them to disk somewhere..
        // Then we #include the C header from Swift
        // And add the Swift file to the project
        // For now let's just write them to a hard coded place on disk and see everything work..
        // then we can use variables instead of hard coded paths.
    }

    unimplemented!()
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

                    let swift = module.generate_swift();
                    generated.swift += &swift;
                    generated.swift += "\n\n";

                    let c_header = module.generate_c_header();
                    generated.c_header += &c_header;
                    generated.c_header += "\n\n";
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

struct Build {
    //
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
