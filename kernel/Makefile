# Building
TARGET := riscv64imac-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin
KERNEL_ASM := $(KERNEL_ELF).asm
FS_TESTCASE_DIR := ../libc/build/
FS_IMG := $(FS_TESTCASE_DIR)/../fs.img

# BOARD
BOARD ?= qemu
SBI ?= rustsbi
BOOTLOADER := default

# KERNEL ENTRY
KERNEL_ENTRY_PA := 0x80200000

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

build: $(KERNEL_BIN)

fs-img: $(APPS)
# @make -C ../user build TEST=$(TEST) CHAPTER=$(CHAPTER) BASE=$(BASE)
	@cd ../easy-fs-fuse && cargo run --target=x86_64-unknown-linux-gnu --release -- -s $(FS_TESTCASE_DIR) -t $(FS_TESTCASE_DIR)

env:
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
	cargo install cargo-binutils --vers ~0.3
	rustup component add rust-src
	rustup component add llvm-tools-preview

$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

kernel:
	@cd ../user && make build
	@cargo build --release
	@cp $(KERNEL_ELF) ../kernel-qemu

clean:
	@cargo clean
	@cd ../user && make clean

run: build
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-kernel ../kernel-qemu \
		-device loader,file=../kernel-qemu,addr=$(KERNEL_ENTRY_PA) \
		-drive file=$(FS_IMG),if=none,format=raw,id=x0 \
		-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
		-smp 1 -m 64M

debug: build
	@tmux new-session -d \
		"qemu-system-riscv64 -machine virt -nographic -bios $(BOOTLOADER) -device loader,file=../kernel-qemu,addr=$(KERNEL_ENTRY_PA) -drive file=$(FS_IMG),if=none,format=raw,id=x0 -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 -smp 1 -m 64M -s -S" && \
		tmux split-window -h "riscv64-unknown-elf-gdb -ex 'file ../kernel-qemu' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && \
		tmux -2 attach-session -d

dbg: build
	qemu-system-riscv64 -machine virt -nographic -bios $(BOOTLOADER) -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) -drive file=$(FS_IMG),if=none,format=raw,id=x0 -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 -s -S

all: build

.PHONY: build env kernel clean fs-img all
