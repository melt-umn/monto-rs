#!/bin/bash

# Runs a demo of Monto.

set -eu

function assertExec() {
	if [[ ! -x "${1}" ]]; then
		echo "Required file \`${1}' not found or not executable." >&2
		echo "Either this script is out of date, or it's being run on a noexec filesystem." >&2
		exit -1
	fi
}

# Build all the components.
cargo build --all --release --manifest-path ../../Cargo.toml

# Define the locations of the components.
TARGET_DIR=../../target/release
BROKER=monto-broker
CLIENT=get-products-for
SERVICE=monto-parenlang

# Check that all the components exist and are executable.
assertExec "${TARGET_DIR}/${BROKER}"
assertExec "${TARGET_DIR}/${CLIENT}"
assertExec "${TARGET_DIR}/${SERVICE}"

# Start tmux.
tmux new-session -d -n client -s monto3-demo "sleep 1 && ${TARGET_DIR}/${CLIENT} || read"
tmux new-window -n broker -t monto3-demo:1 "${TARGET_DIR}/${BROKER} || read"
tmux new-window -n service -t monto3-demo:2 "${TARGET_DIR}/${SERVICE} || read"
tmux attach -t monto3-demo:0
