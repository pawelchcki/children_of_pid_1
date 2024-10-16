
all:
	cargo build --target=x86_64-unknown-linux-musl
	docker build -t rust-musl-docker . -t rust-musl-docker:latest

run:
	docker run --rm -it rust-musl-docker:latest

.PHONY: all
