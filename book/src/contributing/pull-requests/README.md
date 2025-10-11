# Pull Requests

Before merging a pull request into the master branch, a maintainer will rebase
it into a single commit.

The commit's title and body come from the pull request's title and body.

The pull request title serves to summarize the changes. Titles should use 50 or fewer characters.

Pull request bodies should provide a more detailed description of what the pull request has
achieved.
When applicable, include a code snippet that demonstrates what the pull request enables.

Here is an example pull request title and body:
````
Support Swift Option<String> and Option<&str>

This commit adds support for passing `Option<String>` to and from
`extern "Swift"` functions, as well as for passing `Option<&str>` to
extern "Swift" functions.

For example, the following is now possible:

```rust
#[swift_bridge::bridge]
mod ffi {
    extern "Swift" {
        fn opt_string_function(arg: Option<String>) -> Option<String>;

        fn opt_str_function(arg: Option<&str>);
    }
}
```

Note that you can not yet return `-> Option<&str>` from Swift.

This is an uncommon use case, so we're waiting until someone actually
needs it.
````
