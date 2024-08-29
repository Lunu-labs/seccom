TARGET = seccom


.PHONY: build
build:
	@cargo build --release


.PHONY: run
run:
	@cargo run


.PHONY: clean
clean:
	@cargo clean


.PHONY: check
check:
	@cargo check


.PHONY: test
test:
	@cargo test


.PHONY: all
all: build run
