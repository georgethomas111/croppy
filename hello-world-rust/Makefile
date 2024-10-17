cargo-build:
	cargo install -q worker-build && worker-build --release

wrangler-build:
	npx wrangler build

deploy:
	npx wrangler deploy

dev:
	npx wrangler dev
clean:
	rm -rf dist/
build-detailed:
	cargo build --lib --release --target wasm32-unknown-unknown
