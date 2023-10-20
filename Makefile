staking-bank-production: 
	cd ./staking-bank; cargo concordium build -e --out staking_bank.wasm.v1 -- --features production;

staking-bank-development: 
	cd ./staking-bank; cargo concordium build -e --out staking_bank.wasm.v1 -- --features development;

staking-bank-sandbox: 
	cd ./staking-bank; cargo concordium build -e --out staking_bank.wasm.v1 -- --features sandbox;

build-contracts:
	cd registry; cargo concordium build -e --out registry.wasm.v1; cd ../umbrella-feeds; cargo concordium build -e --out umbrella_feeds.wasm.v1; cd ../dummy-contract; cargo concordium build -e --out dummy-contract.wasm.v1;

build-all-production: 
	make build-contracts; make staking-bank-production;

build-all-development:
	make build-contracts; make staking-bank-development;

build-all-sandbox:
	make build-contracts; make staking-bank-sandbox;

build-all:
	make build-contracts; make staking-bank-production; make staking-bank-development; make staking-bank-sandbox;

test-all:
	cd registry; cargo concordium test; cd ../staking-bank; cargo concordium test -- --features development; cd ../umbrella-feeds; cargo concordium test;

fmt-all:
	cd dummy-contract; cargo +nightly-2023-04-01 fmt; cd ../registry; cargo +nightly-2023-04-01 fmt; cd ../staking-bank; cargo +nightly-2023-04-01 fmt; cd ../umbrella-feeds; cargo +nightly-2023-04-01 fmt;

clippy-all:
	cd dummy-contract; cargo clippy --all; cd ../registry; cargo clippy --all; cd ../staking-bank; cargo clippy --all; cd ../umbrella-feeds; cargo clippy --all;
