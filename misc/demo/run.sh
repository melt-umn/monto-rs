#!/bin/bash

# Runs a demo of Monto.

set -eu

cd "$(dirname "${BASH_SOURCE}")"

BUILD_TYPE="${1:-release}";

if [[ "${BUILD_TYPE}" = "release" ]]; then
	CARGOFLAGS+="--release"
else 
	CARGOFLAGS="${CARGOFLAGS:-}"
fi

cargo build --all ${CARGOFLAGS} --manifest-path ../../Cargo.toml

tmux new-session -d -n client -s monto3-demo "sleep 2; ./client.sh ../../target/${BUILD_TYPE}/monto-simple-client; cd ../../target/${BUILD_TYPE}; ${SHELL}"
tmux new-window -n broker -t monto3-demo:1 "sleep 1; ../../target/${BUILD_TYPE}/monto-broker; read"
tmux new-window -n service -t monto3-demo:2 "../../target/${BUILD_TYPE}/monto-parenlang; read"
tmux attach -t monto3-demo:0
