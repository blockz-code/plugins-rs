IGNORE = -i temp/ -i target/ -i target_dist/

.PHONY: dev
dev:
	cargo watch $(IGNORE) --clear -x 'run --example base'


.PHONY: build
build:
	npx -p typescript tsc --declaration --allowJs --emitDeclarationOnly --outDir types \
		src/extensions/core/index.js \
		src/extensions/media/index.js \
		src/extensions/scrape/index.js \