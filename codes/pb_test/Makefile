update_pb:
	protoc -I=pb --rust_opt=expose_fields=true --rust_out=src/idl content.proto 

rustup:
	rustup default nightly-x86_64-apple-darwink
	rustup component add rustfmt
