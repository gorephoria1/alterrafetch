PREFIX ?= /usr/local
BINDIR = $(PREFIX)/bin

all:
	cargo build --release

install:
	mkdir -p $(DESTDIR)$(BINDIR)
	cp target/release/alterrafetch $(DESTDIR)$(BINDIR)/alterrafetch
	chmod +x $(DESTDIR)$(BINDIR)/alterrafetch

uninstall:
	rm -f $(DESTDIR)$(BINDIR)/alterrafetch

clean:
	cargo clean

.PHONY: all install uninstall clean

