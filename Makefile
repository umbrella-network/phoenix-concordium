build-all:
	cd registry; cargo concordium build --out registry.wasm.v1; cd ../staking-bank; cargo concordium build --out staking_bank.wasm.v1; cd ../umbrella-feeds; cargo concordium build --out umbrella_feeds.wasm.v1; cd ../umbrella-feeds-reader; cargo concordium build --out umbrella_feeds_reader.wasm.v1;

test-all:
	cd registry; cargo concordium test; cd ../staking-bank; cargo concordium test; cd ../umbrella-feeds; cargo concordium test; cd ../umbrella-feeds-reader; cargo concordium test;

fmt-all:
	cd registry; cargo +nightly-2023-04-01 fmt; cd ../staking-bank; cargo +nightly-2023-04-01 fmt; cd ../umbrella-feeds; cargo +nightly-2023-04-01 fmt; cd ../umbrella-feeds-reader; cargo +nightly-2023-04-01 fmt;

clippy-all:
	cd registry; cargo clippy --all; cd ../staking-bank; cargo clippy --all; cd ../umbrella-feeds; cargo clippy --all; cd ../umbrella-feeds-reader; cargo clippy --all;

