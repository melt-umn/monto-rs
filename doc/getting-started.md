# Getting Started with Monto3

## Installing everything

### Prereqs

I assume you have Rust, Silver, and tmux installed.
(If you don't have tmux, the demo Makefile won't work, but Monto as a whole will.)

Rust can be installed without root in a few minutes with [rustup.rs](https://rustup.rs/).

### Getting a client

Right now, there are two clients, the command-line client and the Atom plugin.
The Atom plugin is recommended unless you're hacking on the protocol itself.

#### Getting the Atom Plugin

Check [here](https://github.umn.edu/melt/monto-atom#installing).

#### Getting the CLI client

The CLI client is included in this repo; it should get installed as `monto-simple-client` if installing all binaries.
Running `cargo install` when inside this repo should install it to `~/.cargo/bin`.

### Getting the Broker

The broker's in this repo; `cargo install` should install it to `~/.cargo/bin` as `monto-broker`.

### Getting services

There are a few services floating around:

 - [`ableC-monto`](https://github.umn.edu/ringo025/ableC-monto) is a Monto service for ableC.
 - [`monto-cpp`](https://github.umn.edu/ringo025/monto-cpp) has a C preprocessor as a Monto service. Note that currently, this requires running this service on the same machine as the client. This restriction is planned to be lifted in the future.
 - [`monto-example-services`](https://github.com/melt-umn/monto-example-services) includes a few demo services for plain text.
 - [`monto-loctrans`](https://github.umn.edu/ringo025/monto-loctrans) translates the products given by ableC to the standard ones. It's a kludge, but /shrug.

Yes, I know most of these need to be moved to our org on the public GitHub.

#### `ableC-monto`

There's a `demo` dir with a `Makefile`.
There's some paths at the top that need tweaking, then run `make run`.
`ableC-monto` needs `monto-rs` (the broker), `monto-cpp`, and `monto-loctrans` to run (usefully).

#### `monto-cpp`

Just run `cargo install`. Alternatively, the demo `Makefile` will build and set this up.

#### `monto-example-services`

Just run `cargo install`.

#### `monto-loctrans`

Just run `cargo install`. Alternatively, the demo `Makefile` will build and set this up.

## Running things

For now, do `make run` in the demo directory; services are still a bit of a PITA to configure, and that directory has all the config files that are needed.
