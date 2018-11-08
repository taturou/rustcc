all:
	cargo run

test:
	./test.sh

clean:
	cargo clean
	rm -f ./tmp ./tmp.ll

