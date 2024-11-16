init:
	rustup target add aarch64-apple-ios-sim aarch64-apple-ios

build:
	cargo build --release --target aarch64-apple-darwin --features uniffi/cli
	cargo run --features=uniffi/cli --bin uniffi-bindgen generate --library target/aarch64-apple-darwin/release/librust_core.dylib --language python --out-dir python/
	uniffi-bindgen-go -o go/pkg src/rust-core.udl
	cp target/aarch64-apple-darwin/release/librust_core.dylib python/
	cp target/aarch64-apple-darwin/release/librust_core.dylib go/
	cp target/aarch64-apple-darwin/release/librust_core.a go/pkg/lib

	cp bindings.h go/

run-python:
	python python/main.py

run-go:
	$(MAKE) -C go run

all: build run-python run-go


# export PKG_CONFIG_PATH=/opt/homebrew/opt/openssl/lib/pkgconfig/
# export OPENSSL_DIR=/opt/homebrew/opt/openssl/

install-linker:
	brew install FiloSottile/musl-cross/musl-cross
	brew install SergioBenitez/osxct/x86_64-unknown-linux-gnu
	brew tap messense/macos-cross-toolchains
	brew install aarch64-unknown-linux-gnu

# export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=x86_64-unknown-linux-gnu-gcc
# export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc
# export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-unknown-linux-gnu-gcc

# https://gist.github.com/shqld/256e2c4f4b97957fb0ec250cdc6dc463
# https://github.com/messense/homebrew-macos-cross-toolchains
