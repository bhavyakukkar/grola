DATA = ./data
DATA_SUBDIRS = $(shell find $(DATA) -type d)
DATA_FILES = $(shell find $(DATA) -type f -name '*')

TEMPLATES = ./templates
TEMPLATE_FILES = $(shell find $(TEMPLATES) -type f -name '*')

RENDER = ./www

CONFIG_SERVER = config-server.toml
CONFIG_PAGES = config-pages.toml

COMP = cargo build
OUT_DIR = ./target/release
SRC_DIR = ./rust/src
EXEC_NAME = grola


all: parsers pages

handlers: $(TEMPLATES) $(TEMPLATE_FILES)
	#
	# Making handlers for all template files (.html) in $(TEMPLATES),
	# some of which will pull the data files (.toml) in $(DATA)
	#
	cd rust && \
	TEMPLATES_DIR=../$(TEMPLATES) DATA_DIR=../$(DATA) \
		$(COMP) --bin make-parsers -F make-parsers --release && \
		$(OUT_DIR)/make-parsers

server+:
	#
	# Making the handlers and the server using the updated handlers
	# The server will parse data-files in $(DATA)
	# into templates in $(TEMPLATES) upon each request
	# to the routes in $(CONFIG_SERVER) that map to the respective template
	#
	cd rust && \
	TEMPLATES_DIR=../$(TEMPLATES) DATA_DIR=./$(DATA) \
		$(COMP) --bin dynamic-server -F make-parsers,dynamic-server --release && \
		cd .. && ./rust/$(OUT_DIR)/dynamic-server ./config-server.toml

server:
	#
	# Making the server using the last-built handlers
	# The server will parse data-files in $(DATA)
	# into templates in $(TEMPLATES) upon each request
	# to the routes in $(CONFIG_SERVER) that map to the respective template
	#
	cd rust && \
		$(COMP) --bin dynamic-server -F dynamic-server --release && \
		cd .. && ./rust/$(OUT_DIR)/dynamic-server ./config-server.toml

pages+:
	#
	# Making the handlers and the pages using the updates handlers
	# The pages will be created using by parsing the data in $(DATA)
	# into the templates in $(TEMPLATES) for each of the routes,
	# and the respetive attributes specified in $(CONFIG_PAGES)
	#
	cd rust; \
	TEMPLATES_DIR=../$(TEMPLATES) DATA_DIR=../$(DATA) \
		$(COMP) --bin static-render -F make-parsers,static-render --release && \
		RENDER_DIR=../$(RENDER) \
			$(OUT_DIR)/static-render ../config-pages.toml

pages:
	#
	# Making the pages using the last-built handlers
	# The pages will be created using by parsing the data in $(DATA)
	# into the templates in $(TEMPLATES) for each of the routes,
	# and the respetive attributes specified in $(CONFIG_PAGES)
	#
	cd rust; \
		$(COMP) --bin static-render -F static-render --release && \
		RENDER_DIR=../$(RENDER) \
			$(OUT_DIR)/static-render ../config-pages.toml
