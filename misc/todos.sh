#!/bin/bash

# Finds all TODOs, unimplemented!()s, and panic!()s.
# Requires ripgrep.

set -eu

cd "$(dirname "${BASH_SOURCE[0]}")/.."

command -v rg >/dev/null || cargo install ripgrep

rg 'TODO|unimplemented!|panic!' $@
