crux:
	docker run -d -p 3000:3000 --name CruxDB juxt/crux-standalone:20.02-1.7.0-alpha

int:
	cargo test --test db -- --nocapture

unit:
	cargo test --locked  --lib -- --nocapture

test: unit int