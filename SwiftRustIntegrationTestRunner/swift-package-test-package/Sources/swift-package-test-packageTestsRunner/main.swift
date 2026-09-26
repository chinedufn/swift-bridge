import MySwiftPackage

func expect(_ condition: @autoclosure () -> Bool, _ message: String) {
    if !condition() {
        fatalError(message)
    }
}

expect(hello_rust().toString() == "Hello, From Rust!", "Rust string did not match")
expect(SomeStruct(field: 1).field == 1, "Shared struct field did not match")
expect(UnnamedStruct(_0: 1)._0 == 1, "Unnamed shared struct field did not match")
