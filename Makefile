PROG := duck

install: build
	sudo cp ./target/release/$(PROG) /bin/$(PROG)

build:
	cargo build --release
