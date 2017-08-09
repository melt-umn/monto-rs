#!/bin/bash

set -eu

BIN="${1}"

exec ${BIN} 127.0.0.1
