all:
	zokrates compile -i fusion.zok

setup:
	zokrates setup

verifier:
	zokrates export-verifier
	sed -i -e '204,217d' verifier.sol

clean:
	rm abi.json out out.r1cs out.wtns witness verification.key proof.json verifier.sol
