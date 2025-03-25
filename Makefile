.PHONY: build
build:
	@cargo build

.PHONY: fmt release

fmt:
	@cargo fmt

release:
	@cargo build --release
