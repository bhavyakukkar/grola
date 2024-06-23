DATA = ./data
TEMPLATES = ./templates

build:
	cd rust; \
	TEMPLATES_DIR=../$(TEMPLATES) DATA_DIR=../$(DATA) cargo build --release

run:
	cd rust; \
	./target/release/grola
