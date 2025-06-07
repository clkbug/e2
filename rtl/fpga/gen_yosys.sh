#!/bin/bash

set -euo pipefail
set -x

cd "$(dirname "$0")"

YOSYS_SCRIPT="$1"
FILE_LIST="$2"

while read -r FILE; do
    echo "read_verilog -sv $FILE;"
done <"${FILE_LIST}" >"${YOSYS_SCRIPT}"

echo "synth_gowin -top rtl_e2_top -json e2_top.json" >>"${YOSYS_SCRIPT}"
