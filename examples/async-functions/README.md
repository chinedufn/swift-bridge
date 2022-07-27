# async-functions example

In this example we create a Rust function in [src/lib.rs](src/lib.rs) that uses [reqwest][reqwest] in order to make an HTTP request
to an API that returns your public IP address.

We call that function from [main.swift](main.swift) and `await` the fetched data.

## To Run

```
git clone https://github.com/chinedufn/swift-bridge
cd examples/async-functions

./build.sh
./main
```

Here's some example output:

```sh
We're in Swift about to call our async Rust function.
Starting HTTP request from the Rust side...
HTTP request complete. Returning the value to Swift...
Now we're in Swift again. IP address: 123.4.56.7
```

[reqwest]: https://github.com/seanmonstar/reqwest
