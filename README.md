# monto-rs

[![Build Status](https://travis-ci.org/melt-umn/monto-rs.svg?branch=master)](https://travis-ci.org/melt-umn/monto-rs)
[![Build Status](https://ci.appveyor.com/api/projects/status/jsy09v7mqxw2xqex/branch/master?svg=true)](https://ci.appveyor.com/project/remexre/monto-rs/branch/master)

A crate for the Monto protocol. This crate implements version 3.0.0-draft02 of the protocol, which is specified [here](https://melt-umn.github.io/monto-v3-draft/draft02).

## Installing

At some point, I'll set up auto-building binaries, so tags get built as GitHub releases.
Until then, use the "Building from Source" instructions.

### Building from Source

#### Installing Dependencies

This project is written in Rust.
If you don't have Rust and Cargo installed, you can install them via [rustup.rs](https://rustup.rs/) (no root required).
This project does not use any unstable features, so any recent version of the compiler should work.

#### Building and Installing

```
git clone https://github.com/melt-umn/monto-rs.git
cd monto-rs
cargo install
```

This will build `monto-broker` and `monto-simple-client` and copy them to `~/.local/bin` (or your operating system's equivalent).

## Notes

 - This is currently at a "good enough" stage; all the features fundamentally work, assuming clients and services that comply with the specification.
 - This could also use a large-scale reorganization, and the removal of a lot of "TODO Error Handling"s.
 - HTTP/2 support is blocked on [hyperium/hyper#304](https://github.com/hyperium/hyper/issues/304)
