.PHONY: veryl clean format load
veryl:
	veryl build
	${MAKE} -C fpga

load: veryl
	${MAKE} -C fpga load

clean:
	veryl clean
	${MAKE} -C fpga clean

format:
	veryl fmt
