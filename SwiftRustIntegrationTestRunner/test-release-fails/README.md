# Test Release Builds Fail

This test is a reproduction case for when building for release,
previously as `extern "Swift"` functions were `internal`
the Swift compiler would strip these, as it would infer
that they were dead code, even though these need to be accessible
from the Rust side of the FFI.