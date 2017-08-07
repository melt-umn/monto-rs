#!/bin/bash

# Runs a demo of Monto.

set -eu

cargo build --all --manifest-path ../../Cargo.toml
# cargo build --all --release --manifest-path ../../Cargo.toml

tmux new-session -d -n client -s monto3-demo "sleep 1 && ./client.sh || read"
tmux new-window -n broker -t monto3-demo:1 "../../target/debug/monto-broker || read"
tmux new-window -n service -t monto3-demo:2 "../../target/debug/monto-parenlang || read"
tmux attach -t monto3-demo:0
