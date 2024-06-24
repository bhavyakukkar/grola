DATA = ./data
TEMPLATES = ./templates

parsers:
	echo "Making parsers for all template files (.html) in $(TEMPLATES) \
	some of which will pull the data files (.toml) in $(DATA)"; \
	cd rust; \
	TEMPLATES_DIR=../$(TEMPLATES) DATA_DIR=../$(DATA) cargo build --release

pages:
	echo "Generating pages for all template files (.html) in $(TEMPLATES) \
	some of which will be rendered with the data files (.toml) in $(DATA)"; \
	cd rust; \
	TEMPLATES_DIR=../$(TEMPLATES) DATA_DIR=../$(DATA) cargo build --release

server:
	# relative to $(DATA)
	echo "Running the server that will parse data-files in $(DATA) upon each request" \
	cd rust; \
	./target/release/grola
