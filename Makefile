crux:
	docker run -d -p 3000:3000 --name CruxDB juxt/crux-standalone:20.07-1.10.0
	
int:
	cargo test --test db -- --nocapture

unit:
	cargo test --locked  --lib -- --nocapture

test: unit int