rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

install:
	sudo install cargo-lambda

format:
	cargo fmt --quiet

format-check:
	@rustup component add rustfmt 2> /dev/null
	@cargo fmt --all -- --check

build-release:
	@echo "Building release version for platfomr $(shell uname -s)"
	cargo build --release 
	
lint:
	cargo clippy --quiet

test:
	cargo test --quiet

lint:
	cargo clippy --quiet

release:
	cargo build --release

	
run:
	cargo run 

release:
	cargo lambda build --release

release-arm:
	cargo lambda build --release --arm64 --output-format zip

deploy:
	cargo lambda deploy

all: format lint test run
