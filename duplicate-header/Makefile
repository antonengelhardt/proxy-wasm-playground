# Command to build new wasm file, docker-compose up and curl to test
simulate:
	cargo build --target wasm32-wasip1 --release
	docker-compose up
clean:
	docker-compose rm --force
