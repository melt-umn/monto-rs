# monto-rs-c

This is the C bindings to monto-rs.
If you're trying to port monto-rs to a non-C language, you should probably write a client/service implementation in that language rather than binding to these bindings.

The headers for the client and service portions, respectively, are in the `include` directory.

## Installation

monto-rs (and Rust for that matter) do not need to be installed for a binary distribution to work.
Unfortunately, no binary distribution has been set up yet; send a PR to the `.travis.yml` and/or `appveyor.yml` if you want to.
Otherwise, installing from source is supported.

## Installing from source

From the repo root:

```shell
cargo build --release
sudo install -c -m644 target/release/libmonto_rs.a /usr/local/lib
sudo install -c -m644 target/release/libmonto_rs.so /usr/local/lib
sudo install -c -m755 -T monto-rs-c/include/monto_rs /usr/local/include/monto_rs
```

This requires Cargo and a working Rust installation; it is recommended to install Rust from [rustup.rs](https://rustup.rs).
