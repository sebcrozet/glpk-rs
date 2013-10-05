all:
	mkdir -p lib
	rustc --lib --opt-level 3 src/lib.rs --out-dir lib
