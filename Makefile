DATA = ./data
DATA_SUBDIRS = $(shell find $(DATA) -type d)
DATA_FILES = $(shell find $(DATA) -type f -name '*')

TEMPLATES = ./templates
TEMPLATE_FILES = $(shell find $(TEMPLATES) -type f -name '*')

COMP = cargo build
OUT_DIR = ./target/release
SRC_DIR = ./rust/src
EXEC_NAME = grola


all: parsers pages

parsers: $(TEMPLATES) $(TEMPLATE_FILES)
	# Making parsers for all template files (.html) in $(TEMPLATES),
	# some of which will pull the data files (.toml) in $(DATA)
	#
	cd rust && \
	TEMPLATES_DIR=../$(TEMPLATES) DATA_DIR=../$(DATA) \
		$(COMP) --bin make-parsers -F make-parsers --release && \
		$(OUT_DIR)/make-parsers

pages: parsers $(DATA) $(DATA_SUBDIRS) $(DATA_FILES)
	cd rust; \
	TEMPLATES_DIR=../$(TEMPLATES) DATA_DIR=../$(DATA) \
		$(COMP) --release

server:
	# Running the server that will parse data-files in $(DATA) upon each request
	#
	cd rust && \
	TEMPLATES_DIR=../$(TEMPLATES) DATA_DIR=../$(DATA) \
		$(COMP) --bin dynamic-server -F make-parsers,dynamic-server --release && \
		$(OUT_DIR)/dynamic-server

serve:
	# Making the server using the last-built parsers
	# Running the server that will parse data-files in $(DATA) upon each request
	#
	cd rust && \
	DATA_DIR=../$(DATA) \
		$(COMP) --bin dynamic-server -F dynamic-server --release && \
		$(OUT_DIR)/dynamic-server
