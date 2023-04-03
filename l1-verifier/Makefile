all:
	cp ../circuits/verifier.sol ./src/Verifier.sol
	forge clean
	forge bind --select Fusion --crate-name "fusion-l1"
	# TODO remove this when Foundry ships with ethers-rs 2.0.3
	sed -i 's/git = \"https:\/\/github.com\/gakonst\/ethers-rs\"/version = \"2\"/' out/bindings/Cargo.toml

clean:
	forge clean
