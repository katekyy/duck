PROG=duck
TARGET=./target/release

default: install

install-termux: build
	cp $(TARGET)/$(PROG) /data/data/com.termux/files/usr/bin/$(PROG)

install: build
	sudo cp ./target/release/$(PROG) /bin/$(PROG)

build:
	cargo build --release

test: build
	$(TARGET)/$(PROG) "Hello World!" 00:15:00
