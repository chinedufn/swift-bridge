pub const SWIFT_RUST_RESULT: &'static str = r#"
public enum RustResult<T, E> {
    case Ok(T)
    case Err(E)
}

extension RustResult {
    func ok() -> T? {
        switch self {
        case .Ok(let ok):
            return ok
        case .Err(_):
            return nil
        }
    }

    func err() -> E? {
        switch self {
        case .Ok(_):
            return nil
        case .Err(let err):
            return err
        }
    }
    
    func toResult() -> Result<T, E>
    where E: Error {
        switch self {
        case .Ok(let ok):
            return .success(ok)
        case .Err(let err):
            return .failure(err)
        }
    }
}
"#;

pub const C_RESULT_SUPPORT: &'static str = r#"
struct __private__ResultPtrAndPtr { bool is_ok; void* ok_or_err; };
"#;
