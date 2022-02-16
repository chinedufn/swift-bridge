# async-functions example

In this example we create a function in Rust that uses [reqwest] in order to make an HTTP request
to an API that returns your public IP address.

We call that function from Swift and `await` the fetched data.

Here's some example output:

```sh
IP address: 123.4.5.67
```

## Running

```
git clone https://github.com/chinedufn/swift-bridge
cd examples/async-functions
./build.sh
./main
```

[reqwest](https://github.com/seanmonstar/reqwest)
