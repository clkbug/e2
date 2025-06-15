.PHONY: all build run clean

all: build

build:
	verilator --cc -f ../e2_core.f --exe main.cpp --top-module e2_core_e2_top --Mdir obj_dir --trace-fst --trace-params --trace-structs --trace-underscore
	$(MAKE) -C obj_dir/ -f Ve2_core_e2_top.mk


run: build
	./obj_dir/Ve2_core_e2_top

clean:
	rm -rf obj_dir
