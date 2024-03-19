# Test Release

This test makes sure that when building for release,
the Swift compiler doesn't strip functions that need to be accessible
from the Rust side of the FFI.
This test verifies that by making `extern "Swift"` functions `public`
release builds now succeed.