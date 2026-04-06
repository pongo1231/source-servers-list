release:
	cargo build --release
	for bin in target/release/* target/*/release/*; do \
		if [ -f "$bin" ] && file "$bin" | grep -q "executable"; then \
			upx --best --lzma "$bin"; \
		fi; \
	done