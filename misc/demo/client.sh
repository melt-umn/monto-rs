#!/bin/bash

set -eu

BIN="${1}"

echo -e "\x1b[1;33mListing products...\x1b[0m"
${BIN} -q list
read

echo -e "\n\x1b[1;33mFetching depth product...\x1b[0m"
${BIN} -q fetch \
	edu.umn.cs.melt.monto.parenlang \
	edu.umn.cs.melt.monto_rs.balanced_parens.depth \
	balanced-parens \
	example.balparens \
	example.balparens
read

echo -e "\n\x1b[1;33mHighlighting code...\x1b[0m"
${BIN} -q fetch \
	edu.umn.cs.melt.monto.parenlang \
	highlighting \
	balanced-parens \
	example.balparens \
	example.balparens
read
