.PHONY: all veryl clean format load

all: veryl
	${MAKE} -C fpga

veryl:
	veryl build

load: veryl
	${MAKE} -C fpga load

clean:
	veryl clean
	${MAKE} -C fpga clean

format:
	veryl fmt
