.PHONY: run
run:
	apt update && apt install -y iproute2 iputils-ping
	cargo run
