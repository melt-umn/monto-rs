#!/bin/bash

# Stress-tests the Broker for negotiations.

set -eu

function report() {
	vegeta report -inputs ../report/stress_test.bin -output "../report/stress_test.${2}" -reporter "${1}"
}

cd "$(dirname "${BASH_SOURCE[0]}")/data"

command -v vegeta >/dev/null || go get -u github.com/tsenart/vegeta

mkdir -p "../report"
echo "Stress testing for 60 seconds..."
vegeta attack -duration 60s -rate ${1:-1000} -targets stress_test.targets -output ../report/stress_test.bin

echo "Creating reports..."
report text txt
report json json
report plot html
report 'hist[0,500us,1ms,2ms,5ms]' hist.txt
