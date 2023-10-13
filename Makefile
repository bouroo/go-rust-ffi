CURRENT_DIR := $(pwd || echo ${PWD})

.PHONY: build-lib
build-lib:
	@cd lib/rs-qr && cargo build --release
	@cp lib/rs-qr/target/rs-qr.h ./lib
	@cp lib/rs-qr/target/release/librs_qr.a ./lib
	@cp lib/rs-qr/target/release/librs_qr.so ./lib

.PHONY: build-go
build-go: build-lib
	go build -ldflags="-r $(CURRENT_DIR)lib" -o dist/ ./...

.PHONY: run-go
run-go: build-go
	dist/go-rust-ffi