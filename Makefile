all:
	cargo build

test:
	./test.sh

clean:
	cargo clean
	rm -f ./tmp ./tmp.ll

