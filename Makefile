.PHONY: clean
clean:
	cargo clean

.PHONY: build
build:
	cargo build --workspace

.PHONY: rebuild
rebuild: clean build

.PHONY: test
test:
	cargo test

.PHONY: benchmarks
benchmarks:
	chmod +x ./benchmarks.sh
	./benchmarks.sh

.PHONY: clippy
clippy:
	cargo clippy


.PHONY: ci
ci: rebuild test clippy
