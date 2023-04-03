.PHONY: run
run:
	# apt update && apt install -y iproute2 iputils-ping
	cargo run
	# /src/target/debug/tcp &
	# pid=$!

	# # set up tun0 nic
	# ip addr add 192.168.0.1/24 dev tun0
	# ip link set up dev tun0

	# # send a packet to tun0
	# ping -I tun0 192.168.0.5

	# wait $pid
