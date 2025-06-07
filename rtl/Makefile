.PHONY: veryl clean format
veryl:
	veryl build
	${MAKE} -C fpga

clean:
	veryl clean
	${MAKE} -C fpga clean

format:
	veryl fmt
