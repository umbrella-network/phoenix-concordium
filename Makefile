build-all:
	cd registry; cargo concordium build --out registry.wasm.v1; cd ../umbrella-feeds-factory; cargo concordium build --out umbrella_feeds_factory.wasm.v1; cd ../staking-bank; cargo concordium build --out staking_bank.wasm.v1; cd ../umbrella-feeds; cargo concordium build --out umbrella_feeds.wasm.v1; cd ../umbrella-feeds-reader; cargo concordium build --out umbrella_feeds_reader.wasm.v1;

test-all:
	cd registry; cargo concordium test; cd ../umbrella-feeds-factory; cargo concordium test; cd ../staking-bank; cargo concordium test; cd ../umbrella-feeds; cargo concordium test; cd ../umbrella-feeds-reader; cargo concordium test;

