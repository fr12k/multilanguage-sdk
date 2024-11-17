.EXPORT_ALL_VARIABLES:

# OPENSSL_DIR=/opt/homebrew/Cellar/openssl\@3/3.4.0/
# OPENSSL_INCLUDE_DIR=/opt/homebrew/Cellar/openssl\@3/3.4.0/include/openssl

CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=x86_64-unknown-linux-gnu-gcc
CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc
CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-unknown-linux-gnu-gcc

init:
	cargo install cargo-post

build:
	# cargo post build --release --target aarch64-apple-darwin
	# cargo post build --release --target x86_64-apple-darwin
	cargo post build --release --target x86_64-unknown-linux-gnu
	cargo post build --release --target aarch64-unknown-linux-gnu
	cargo post build --release --target x86_64-unknown-linux-musl
	cargo post build --release --target aarch64-unknown-linux-musl

	# cargo run --features=uniffi/cli --bin uniffi-bindgen generate --library target/aarch64-apple-darwin/release/librust_core.dylib --language python --out-dir python/
	# uniffi-bindgen-go -o go/pkg src/rust-core.udl
	cp target/aarch64-apple-darwin/release/librust_core.dylib python/
	cp target/aarch64-apple-darwin/release/librust_core.dylib go/
	cp target/aarch64-apple-darwin/release/librust_core.a go/pkg/lib

	cp bindings.h go/

run-python:
	python python/main.py

run-go:
	$(MAKE) -C go run

all: init rustup install-linker build run-python run-go


# export PKG_CONFIG_PATH=/opt/homebrew/opt/openssl/lib/pkgconfig/
# export OPENSSL_DIR=/opt/homebrew/opt/openssl/

install-linker:
	brew install FiloSottile/musl-cross/musl-cross
	brew install SergioBenitez/osxct/x86_64-unknown-linux-gnu
	brew tap messense/macos-cross-toolchains
	brew install aarch64-unknown-linux-gnu

rustup:
	rustup target add aarch64-apple-darwin
	rustup target add x86_64-apple-darwin
	rustup target add x86_64-unknown-linux-gnu
	rustup target add aarch64-unknown-linux-gnu
	rustup target add x86_64-unknown-linux-musl
	rustup target add aarch64-unknown-linux-musl


# export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=x86_64-unknown-linux-gnu-gcc
# export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc
# export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-unknown-linux-gnu-gcc

# https://gist.github.com/shqld/256e2c4f4b97957fb0ec250cdc6dc463
# https://github.com/messense/homebrew-macos-cross-toolchains
