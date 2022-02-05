# Building

Writing an application that uses Swift and Rust essentially boils
down to taking some Swift source code and some Rust source code and
compiling it all into a final binary.

If there was a super-compiler that knew about both Swift and Rust code you
could tell that compiler to compile source files from both languages into
a final binary.

This doesn't exist, so instead you need to use both a Swift compiler and
a Rust compiler in a two-stepped approach.

First you compile one of the languages into a native library.

Then you compile the other language into your final binary, but
while doing so you tell the second compiler to link in the native library
that you created.

Here's how this process would look if you were linking a Rust native
library into a Swift executable.

```text
┌──────────────────────────────────┐           ┌───────────────────┐       
│// Rust code                      │           │// Swift Code      │       
│                                  │           │                   │       
│pub extern "C" fn rust_hello() {  │           │rust_hello()       │       
│    println!("Hi, I'm Rust!")     │           │                   │       
│}                                 │           │                   │       
└──────────────────────────────────┘           └───────────────────┘       
                 │                                       │                 
    Compile Rust │                                       │ Compile Swift to
   to native lib │                                       │ executable      
                 │                                       │                 
                 ▼                      Link in Rust     │                 
┌────────────────────────────────┐      native lib       │                 
│       libmy_rust_crate.a       │───────────────────────┤                 
└────────────────────────────────┘                       │                 
                                                         │                 
                                                         ▼                 
                                       ┌──────────────────────────────────┐
                                       │     Final Executable Binary      │
                                       │                                  │
                                       └──────────────────────────────────┘
```

In a similar fashion, you could also compile Swift code into a native library and then
link it in when compiling your Rust code.

Which direction you choose largely comes down to whichever is easier based on the
build tools that you already use or plan to use.

This chapter walks you through a few different ways to build Swift and Rust code.
