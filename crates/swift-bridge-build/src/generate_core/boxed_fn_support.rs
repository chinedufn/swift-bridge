/// Declares support types for callbacks that have no arguments and don't return a value.
///
/// Support types for callbacks that have arguments or return a value are generated dynamically
/// when generating code for bridged functions.
pub const SWIFT_CALLBACK_SUPPORT_NO_ARGS_NO_RETURN: &str = r#"
public class __private__RustFnOnceCallbackNoArgsNoRet {
    var ptr: UnsafeMutableRawPointer
    var called = false

    init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }

    deinit {
        if !called {
            __swift_bridge__$free_boxed_fn_once_no_args_no_return(ptr)
        }
    }

    func call() {
        if called {
            fatalError("Cannot call a Rust FnOnce function twice")
        }
        called = true
        return __swift_bridge__$call_boxed_fn_once_no_args_no_return(ptr)
    }
}
"#;

pub const C_CALLBACK_SUPPORT_NO_ARGS_NO_RETURN: &str = r#"
void __swift_bridge__$call_boxed_fn_once_no_args_no_return(void* boxed_fnonce);
void __swift_bridge__$free_boxed_fn_once_no_args_no_return(void* boxed_fnonce);
"#;
