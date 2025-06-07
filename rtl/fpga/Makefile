BOARD=tangnano9k
FAMILY=GW1N-9C
DEVICE=GW1NR-LV9QN88PC6/I5

all: e2_top.fs

# Synthesis
e2_top.json: ../target/e2_top.sv
	yosys -p "read_verilog -sv ../target/e2_top.sv; synth_gowin -top rtl_e2_top -json e2_top.json"

# Place and Route
e2_top_pnr.json: e2_top.json
	nextpnr-himbaechel --json e2_top.json --write e2_top_pnr.json --freq 27 --device ${DEVICE} --vopt family=${FAMILY} --vopt cst=${BOARD}.cst

# Generate Bitstream
e2_top.fs: e2_top_pnr.json
	gowin_pack -d ${FAMILY} -o e2_top.fs e2_top_pnr.json

# Program Board
load: e2_top.fs
	openFPGALoader -b ${BOARD} e2_top.fs -f

.PHONY: load
.INTERMEDIATE: e2_top_pnr.json e2_top.json
