kernel:
	$(MAKE) -C kernel build

libc:
	$(MAKE) -C libc all

all: libc kernel

.PHONY: all libc kernel
