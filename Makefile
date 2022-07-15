test:
	cargo test -- --test-threads=1
watch:
	cargo watch -s "cargo test -- --test-threads=1"
clean:
	rm -rf ./target
