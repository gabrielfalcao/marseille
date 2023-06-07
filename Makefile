.PHONY: all clean cls release debug fix fmt check build test examples run

INSTALL_PATH		:= $${HOME}/usr/bin/

MORSEL_NAME		:=morsel
MORSEL_DEBUG_BIN	:=target/debug/$(MORSEL_NAME)
MORSEL_RELEASE_BIN	:=target/release/$(MORSEL_NAME)
MORSEL_BIN		:=$(MORSEL_DEBUG_BIN)
MORSEL_RUN		:=cargo run --bin $(MORSEL_NAME)

all: test debug release

$(INSTALL_PATH):
	mkdir -p $@
	@2>&1 ggrep -P '[$$]?[{]?HOME[}]?/usr/bin' ~/.bashrc > /dev/null || echo 'export PATH="${PATH}:$HOME/usr/bin"' | tee -a ~/.bashrc

$(MORSEL_RELEASE_BIN): $(INSTALL_PATH)
	cargo build --release

$(MORSEL_DEBUG_BIN): $(INSTALL_PATH)
	cargo build

release: check fix | $(MORSEL_RELEASE_BIN)
	install $(MORSEL_RELEASE_BIN) $(INSTALL_PATH)

debug: check fix | $(MORSEL_DEBUG_BIN)
	install $(MORSEL_DEBUG_BIN) $(INSTALL_PATH)

clean: cls
	@rm -rf target

cls:
	-@reset || tput reset

fix:
	cargo fix --allow-dirty --allow-staged

fmt:
	rustfmt --edition 2021 src/*.rs

check:
	cargo check --all-targets

run build test: check
	cargo $@
