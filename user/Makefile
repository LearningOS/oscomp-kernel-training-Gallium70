TARGET := riscv64imac-unknown-none-elf

build: rust-user

rust-user:
# rustup target add $(TARGET)
	@cargo build --release

clean:
	@cargo clean

.PHONY: build rust-user clean
