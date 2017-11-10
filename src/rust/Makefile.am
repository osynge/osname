# Cargo binding

EXTRA_DIST = Cargo.toml \
    src/importer.rs \
    src/lib.rs \
    build.rs \
	$(null)



all-local:
	cargo build @CARGO_RELEASE_ARGS@

clean-local:
	-cargo clean

install-exec-local:
	$(MKDIR_P) $(DESTDIR)$(libdir)
	$(INSTALL) -c -m 755 target/@CARGO_TARGET_DIR@/libosnamelib.so $(DESTDIR)$(libdir)

DISTCLEANFILES =                                \
	$(desktop_files)                        \
	$(null)


check:
	cargo check
	cargo test
