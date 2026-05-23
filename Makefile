.PHONY: all build deb tarball clean

all: build deb tarball

build:
	cargo build --release

deb: build
	cargo deb
	mkdir -p dist
	cp target/debian/tick_0.1.0-1_amd64.deb dist/

tarball: build
	mkdir -p dist/tick
	cp target/release/tick dist/tick/
	cp assets/tick.desktop dist/tick/
	cp assets/tick.svg dist/tick/
	cp LICENSE dist/tick/
	cd dist && tar -czf tick-0.1.0-x86_64-linux.tar.gz tick
	rm -rf dist/tick

clean:
	cargo clean
	rm -rf dist target/debian
