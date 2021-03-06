TARGET = target/release
BINARY = main
PREFIX = /usr

main: rvrrpd-pw
	@cargo build --release

test:
	@cargo test

docs:
	@cargo doc --no-deps

check:
	@cargo fmt --all -- --check

clean: rvrrpd-pw-clean
	@cargo clean

install: rvrrpd-pw-install
	[ ! -d "$(DESTDIR)$(PREFIX)/sbin" ] && mkdir -p "$(DESTDIR)$(PREFIX)/sbin"
	cp $(TARGET)/${BINARY} $(DESTDIR)$(PREFIX)/sbin/rvrrpd
	chmod 755 $(DESTDIR)$(PREFIX)/sbin/rvrrpd
	[ ! -d "$(DESTDIR)/etc/rvrrpd" ] && mkdir -p "$(DESTDIR)/etc/rvrrpd"

rvrrpd-pw:
	cd utils/rvrrpd-pw && $(MAKE)

rvrrpd-pw-install:
	cd utils/rvrrpd-pw && $(MAKE) install

rvrrpd-pw-clean:
	cd utils/rvrrpd-pw && $(MAKE) clean

.PHONY: main test docs check clean install
