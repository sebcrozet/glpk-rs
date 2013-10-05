all:
	mkdir -p lib
	rustc --lib --opt-level 3 src/glpk.rs --out-dir lib
