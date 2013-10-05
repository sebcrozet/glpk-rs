tmp=_git_distcheck

all:
	mkdir -p lib
	rustc --lib --opt-level 3 src/lib.rs --out-dir lib

test:
	mkdir -p lib
	RUST_TEST_TASKS=1 rust test src/lib.rs
	rm libtest~

distcheck:
	rm -rf $(tmp)
	git clone --recursive . $(tmp)
	make -C $(tmp)
	make -C $(tmp) test
	rm -rf $(tmp)
