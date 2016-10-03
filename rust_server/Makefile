proto:
	protoc --rust_out=src -I ../. ../*.proto
	protoc --rust-grpc_out=src -I ../. ../*.proto

build: proto
	cargo build

run: proto
	cargo run --release
