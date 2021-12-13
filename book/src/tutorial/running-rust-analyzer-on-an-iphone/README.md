# Tutorial: Running rust-analyzer on an iPhone

In this chapter we'll create a new iOS application that makes use of `swift-bridge` in order
use `rust-analyzer` to perform syntax highlighting of Rust code.

## Project Setup

Create a new project.

```sh
cargo new --lib rust-analyzer-ios
cd rust-analyzer-ios
```

---

In the `Cargo.toml`, set the crate-type and build script.

```toml
# Show the full Cargo.toml with crate-type staticlib and build = "build.rs"
```

---

Install [`cargo-lipo`](https://github.com/TimNN/cargo-lipo).

```
cargo install -f cargo-lipo
```

---

Create a bash script that we can use to build the application

```sh
# Bash script to build using cargo lipo here..
```

---

Create a new Xcode project within the `rust-analyzer-ios` directory called `RustAnalyzerIos`.

TODO: Screenshots of creating the Xcode project here.

Your working directory should now look like this:

```sh
# TODO: Show ls of directory here
```

---

Create a new bridging header

---

Edit the NAME_OF_SETTINGS_FILE to use the bridging header

---

Edit the NAME_OF_SETTINGS_FILE to link to your Rust library

---

Edit the NAME_OF_SETTINGS_FILE to add a build phase that calls your run script.
