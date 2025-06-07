BOARD=tangnano9k
FAMILY=GW1N-9C
DEVICE=GW1NR-LV9QN88PC6/I5

all: e2_top.fs

# Synthesis
.PHONY: e2_top.json
e2_top.json: #../rtl.f
	./gen_yosys.sh yosys.ys ../rtl.f
	yosys -s yosys.ys
# yosys -p "read_verilog -sv ../rtl.f; synth_gowin -top rtl_e2_top -json e2_top.json"

# Place and Route
e2_top_pnr.json: e2_top.json
	nextpnr-himbaechel --json e2_top.json --write e2_top_pnr.json --freq 1 --device ${DEVICE} --vopt family=${FAMILY} --vopt cst=${BOARD}.cst

# Generate Bitstream
e2_top.fs: e2_top_pnr.json
	gowin_pack -d ${FAMILY} -o e2_top.fs e2_top_pnr.json

# Program Board
load: e2_top.fs
	openFPGALoader -b ${BOARD} e2_top.fs -f

.PHONY: load clean
# .INTERMEDIATE: e2_top_pnr.json e2_top.json

clean:
	rm -f e2_top.fs e2_top.json e2_top_pnr.json
