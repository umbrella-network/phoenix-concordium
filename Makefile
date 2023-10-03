build-all:
	cd registry; cargo concordium build -e --out registry.wasm.v1; cd ../staking-bank; cargo concordium build -e --out staking_bank.wasm.v1 -- --features production; cd ../umbrella-feeds; cargo concordium build -e --out umbrella_feeds.wasm.v1; cd ../umbrella-feeds-reader; cargo concordium build -e --out umbrella_feeds_reader.wasm.v1; cd ../dummy-contract; cargo concordium build -e --out dummy-contract.wasm.v1;

test-all:
	cd registry; cargo concordium test; cd ../staking-bank; cargo concordium test -- --features production; cd ../umbrella-feeds; cargo concordium test; cd ../umbrella-feeds-reader; cargo concordium test;

fmt-all:
	cd dummy-contract; cargo +nightly-2023-04-01 fmt; cd ../registry; cargo +nightly-2023-04-01 fmt; cd ../staking-bank; cargo +nightly-2023-04-01 fmt; cd ../umbrella-feeds; cargo +nightly-2023-04-01 fmt; cd ../umbrella-feeds-reader; cargo +nightly-2023-04-01 fmt;

clippy-all:
	cd dummy-contract; cargo clippy --all; cd ../registry; cargo clippy --all; cd ../staking-bank; cargo clippy --all; cd ../umbrella-feeds; cargo clippy --all; cd ../umbrella-feeds-reader; cargo clippy --all;

