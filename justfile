release target="":
	cargo b --release {{ if target != "" { "--target " + target } else { "" } }}
	for bin in target/release/* target/*/release/*; do \
		if [ -f "$bin" ] && file "$bin" | grep -q "executable"; then \
			upx --best --lzma "$bin"; \
		fi; \
	done
