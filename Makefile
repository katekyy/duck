PROG := duck

default: install

install-termux: build
	cp ./target/release/$(PROG) /data/data/com.termux/files/usr/bin/$(PROG)

install: build
	sudo cp ./target/release/$(PROG) /bin/$(PROG)

build:
	cargo build --release
