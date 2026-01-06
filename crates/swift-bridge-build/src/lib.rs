//! Parse Rust source files for #\[swift_bridge::bridge\] modules and #\[swift_bridge::bridged\]
//! structs and then generate the corresponding C header files and Swift files.
//!
//! This crate also supports conditional attributes via `cfg_attr`:
//! - `#[cfg_attr(target_os = "macos", swift_bridge::bridge)]`
//! - `#[cfg_attr(target_os = "macos", swift_bridge::bridged)]`

#![deny(missing_docs)]

mod package;
use crate::generate_core::write_core_swift_and_c;
pub use package::*;
use std::path::Path;
use swift_bridge_ir::bridged_struct::generate_bridged_swift_and_c;
use swift_bridge_ir::{CodegenConfig, SwiftBridgeModule};
use syn::__private::ToTokens;
use syn::{Attribute, File, Item};

mod generate_core;

/// Check if an attribute matches the target, either directly or inside a `cfg_attr`.
///
/// This handles both:
/// - `#[swift_bridge::bridged]`
/// - `#[cfg_attr(condition, swift_bridge::bridged)]`
fn is_swift_bridge_attribute(attr: &Attribute, target: &str) -> bool {
    let path_str = attr.path.to_token_stream().to_string();

    // Direct attribute match
    if path_str == format!("swift_bridge :: {}", target)
        || path_str == format!("swift_bridge_macro :: {}", target)
    {
        return true;
    }

    // Check for cfg_attr containing our target attribute
    if path_str == "cfg_attr" {
        let tokens = attr.tokens.to_string();
        // Look for swift_bridge::target or swift_bridge_macro::target inside cfg_attr
        if tokens.contains(&format!("swift_bridge :: {}", target))
            || tokens.contains(&format!("swift_bridge_macro :: {}", target))
        {
            return true;
        }
    }

    false
}

/// Parse rust sources files for `#\[swift_bridge::bridge\]` modules and `#\[swift_bridge::bridged\]`
/// structs and generate the corresponding Swift and C header code.
///
/// This function automatically detects both:
/// - `#[swift_bridge::bridge]` modules
/// - `#[swift_bridge::bridged]` structs
///
/// # Example
///
/// ```ignore
/// // In build.rs
/// swift_bridge_build::parse_bridges(vec!["src/lib.rs"])
///     .write_all_concatenated("generated", "my_crate");
/// ```
pub fn parse_bridges(
    rust_source_files: impl IntoIterator<Item = impl AsRef<Path>>,
) -> GeneratedCode {
    let mut generated_code = GeneratedCode::new();

    for rust_file in rust_source_files.into_iter() {
        let rust_file: &Path = rust_file.as_ref();

        let file = std::fs::read_to_string(rust_file).unwrap();
        let gen = match parse_file_contents(&file) {
            Ok(generated) => generated,
            Err(e) => {
                // TODO: Return an error...
                panic!(
                    r#"
Error while parsing {:?}
{}
"#,
                    rust_file, e
                )
            }
        };

        // Collect bridged struct code from this file
        for swift_code in &gen.bridged_swift {
            generated_code.bridged_swift.push(swift_code.clone());
        }
        for c_header in &gen.bridged_c_header {
            generated_code.bridged_c_header.push(c_header.clone());
        }

        generated_code.generated.push(gen);
    }

    generated_code
}

/// Generated Swift files and C headers.
pub struct GeneratedCode {
    generated: Vec<GeneratedFromSwiftBridgeModule>,
    /// Additional Swift code from bridged structs
    bridged_swift: Vec<String>,
    /// Additional C header code from bridged structs
    bridged_c_header: Vec<String>,
}

impl GeneratedCode {
    fn new() -> Self {
        GeneratedCode {
            generated: vec![],
            bridged_swift: vec![],
            bridged_c_header: vec![],
        }
    }

    /// Add Swift code from a `#[swift_bridge::bridged]` struct.
    ///
    /// This method is used to collect the generated Swift code from structs
    /// annotated with `#[swift_bridge::bridged]`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // In your crate
    /// #[swift_bridge::bridged]
    /// pub struct MyStruct {
    ///     pub value: u32,
    /// }
    /// // This generates: pub const __SWIFT_BRIDGE_BRIDGED_SWIFT_MYSTRUCT: &str = "...";
    /// // This generates: pub const __SWIFT_BRIDGE_BRIDGED_HEADER_MYSTRUCT: &str = "...";
    ///
    /// // In build.rs
    /// swift_bridge_build::parse_bridges(vec!["src/lib.rs"])
    ///     .with_bridged_swift(my_crate::__SWIFT_BRIDGE_BRIDGED_SWIFT_MYSTRUCT)
    ///     .with_bridged_c_header(my_crate::__SWIFT_BRIDGE_BRIDGED_HEADER_MYSTRUCT)
    ///     .write_all_concatenated("generated", "my_crate");
    /// ```
    pub fn with_bridged_swift(mut self, swift_code: &str) -> Self {
        self.bridged_swift.push(swift_code.to_string());
        self
    }

    /// Add C header code from a `#[swift_bridge::bridged]` struct.
    ///
    /// See `with_bridged_swift` for usage example.
    pub fn with_bridged_c_header(mut self, c_header_code: &str) -> Self {
        self.bridged_c_header.push(c_header_code.to_string());
        self
    }

    /// Add both Swift and C header code from a `#[swift_bridge::bridged]` struct.
    ///
    /// This is a convenience method that combines `with_bridged_swift` and `with_bridged_c_header`.
    pub fn with_bridged_type(self, swift_code: &str, c_header_code: &str) -> Self {
        self.with_bridged_swift(swift_code)
            .with_bridged_c_header(c_header_code)
    }
}

impl GeneratedCode {
    /// Write all of the generated Swift to a single Swift file and all of the generated C headers
    /// to a single header file.
    pub fn write_all_concatenated(&self, swift_bridge_out_dir: impl AsRef<Path>, crate_name: &str) {
        let swift_bridge_out_dir = swift_bridge_out_dir.as_ref();

        let concatenated_swift = self.concat_swift();
        let concatenated_c = self.concat_c();

        let out = swift_bridge_out_dir.join(&crate_name);
        match std::fs::create_dir_all(&out) {
            Ok(_) => {}
            Err(_) => {}
        };

        std::fs::write(out.join(format!("{}.h", crate_name)), concatenated_c).unwrap();
        std::fs::write(
            out.join(format!("{}.swift", crate_name)),
            concatenated_swift,
        )
        .unwrap();

        write_core_swift_and_c(swift_bridge_out_dir.as_ref());
    }

    /// Concatenate all of the generated Swift code into one file.
    pub fn concat_swift(&self) -> String {
        let mut swift = "".to_string();

        // Add bridged struct Swift code first
        for bridged_swift in &self.bridged_swift {
            swift += bridged_swift;
            swift += "\n\n";
        }

        // Then add bridge module Swift code
        for gen in &self.generated {
            swift += &gen.swift;
        }

        swift
    }

    /// Concatenate all of the generated C code into one file.
    pub fn concat_c(&self) -> String {
        let mut c_header = "".to_string();

        // Add bridged struct C header code first
        for bridged_c in &self.bridged_c_header {
            c_header += bridged_c;
            c_header += "\n";
        }

        // Then add bridge module C header code
        for gen in &self.generated {
            c_header += &gen.c_header;
        }

        c_header
    }
}

fn parse_file_contents(file: &str) -> syn::Result<GeneratedFromSwiftBridgeModule> {
    let file: File = syn::parse_str(file)?;

    let mut generated = GeneratedFromSwiftBridgeModule {
        c_header: "".to_string(),
        swift: "".to_string(),
        bridged_swift: vec![],
        bridged_c_header: vec![],
    };

    for item in file.items {
        match item {
            Item::Mod(module) => {
                // TODO: Move this check into the `impl Parse for SwiftBridgeModule`.. Modify our
                //  tests in swift-bridge-ir to annotate modules with `#[swift_bridge::bridge]`
                if module
                    .attrs
                    .iter()
                    .any(|a| is_swift_bridge_attribute(a, "bridge"))
                {
                    let module: SwiftBridgeModule = syn::parse2(module.to_token_stream())?;

                    let config = CodegenConfig {
                        crate_feature_lookup: Box::new(|feature_name| {
                            let normalized_feature_name = feature_name.replace("-", "_");
                            let normalized_feature_name = normalized_feature_name.to_uppercase();

                            let env_var_name = format!("CARGO_FEATURE_{}", normalized_feature_name);
                            std::env::var(env_var_name).is_ok()
                        }),
                    };
                    let swift_and_c = module.generate_swift_code_and_c_header(config);

                    generated.c_header += &swift_and_c.c_header;
                    generated.c_header += "\n\n";

                    let swift = &swift_and_c.swift;
                    generated.swift += &swift;
                    generated.swift += "\n\n";
                }
            }
            Item::Struct(item_struct) => {
                // Check for #[swift_bridge::bridged] attribute (including cfg_attr)
                if item_struct
                    .attrs
                    .iter()
                    .any(|a| is_swift_bridge_attribute(a, "bridged"))
                {
                    // Note: swift_name attribute is processed by the macro at compile time.
                    // The build script just needs to detect the struct, not parse attributes.
                    if let Some(codegen) = generate_bridged_swift_and_c(&item_struct, None) {
                        generated.bridged_swift.push(codegen.swift);
                        generated.bridged_c_header.push(codegen.c_header);
                    }
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
    bridged_swift: Vec<String>,
    bridged_c_header: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_direct_bridged_attribute() {
        let source = r#"
            #[swift_bridge::bridged]
            pub struct TestStruct {
                pub value: u32,
            }
        "#;

        let result = parse_file_contents(source).unwrap();
        assert_eq!(result.bridged_swift.len(), 1);
        assert_eq!(result.bridged_c_header.len(), 1);
    }

    #[test]
    fn detects_cfg_attr_bridged_attribute() {
        let source = r#"
            #[cfg_attr(target_os = "macos", swift_bridge::bridged)]
            pub struct TestStruct {
                pub value: u32,
            }
        "#;

        let result = parse_file_contents(source).unwrap();
        assert_eq!(result.bridged_swift.len(), 1);
        assert_eq!(result.bridged_c_header.len(), 1);
    }

    #[test]
    fn detects_cfg_attr_bridge_module() {
        // This test just verifies the attribute detection, not full module parsing
        let attr: syn::Attribute =
            syn::parse_quote!(#[cfg_attr(target_os = "macos", swift_bridge::bridge)]);
        assert!(is_swift_bridge_attribute(&attr, "bridge"));
    }

    #[test]
    fn ignores_unrelated_cfg_attr() {
        let source = r#"
            #[cfg_attr(target_os = "macos", derive(Debug))]
            pub struct TestStruct {
                pub value: u32,
            }
        "#;

        let result = parse_file_contents(source).unwrap();
        assert_eq!(result.bridged_swift.len(), 0);
        assert_eq!(result.bridged_c_header.len(), 0);
    }
}
