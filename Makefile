staking-bank-production: 
	cd ./staking-bank; cargo concordium build -e --out staking_bank.wasm.v1 -- --features production;

staking-bank-development: 
	cd ./staking-bank; cargo concordium build -e --out staking_bank.wasm.v1 -- --features development;

staking-bank-sandbox: 
	cd ./staking-bank; cargo concordium build -e --out staking_bank.wasm.v1 -- --features sandbox;

staking-bank-local: 
	cd ./staking-bank; cargo concordium build -e --out staking_bank.wasm.v1 -- --features local;

oracle-integration-production: 
	cd ./oracle-integration; cargo concordium build -e --out oracle_integration.wasm.v1 -- --features production;

oracle-integration-development: 
	cd ./oracle-integration; cargo concordium build -e --out oracle_integration.wasm.v1 -- --features development;

oracle-integration-local: 
	cd ./oracle-integration; cargo concordium build -e --out oracle_integration.wasm.v1 -- --features local;

build-contracts:
	cd registry; cargo concordium build -e --out registry.wasm.v1; cd ../umbrella-feeds; cargo concordium build -e --out umbrella_feeds.wasm.v1; cd ../dummy-contract; cargo concordium build -e --out dummy_contract.wasm.v1;

build-all-production: build-contracts staking-bank-production oracle-integration-production

build-all-development: build-contracts staking-bank-development oracle-integration-development

build-all-sandbox: build-contracts staking-bank-sandbox

build-all-local: build-contracts staking-bank-local oracle-integration-local

build-all: build-contracts staking-bank-production staking-bank-sandbox staking-bank-development staking-bank-local oracle-integration-production oracle-integration-development oracle-integration-local

test-all: build-all-local; cd registry; cargo concordium test; cd ../staking-bank; cargo concordium test; cd ../umbrella-feeds; cargo concordium test; cd ../oracle-integration; cargo concordium test;

fmt-all:
	cd dummy-contract; cargo +nightly-2023-04-01 fmt; cd ../registry; cargo +nightly-2023-04-01 fmt; cd ../staking-bank; cargo +nightly-2023-04-01 fmt; cd ../umbrella-feeds; cargo +nightly-2023-04-01 fmt; cd ../oracle-integration; cargo +nightly-2023-04-01 fmt

clippy-all:
	cd dummy-contract; cargo clippy --all; cd ../registry; cargo clippy --all; cd ../staking-bank; cargo clippy --all; cd ../umbrella-feeds; cargo clippy --all; cd ../oracle-integration; cargo clippy --all
