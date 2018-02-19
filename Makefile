11-crates-library:
	rustc --crate-type=lib src/11-crates-library.rs

11-crates-executable:
	rustc src/11-crates-executable.rs --extern rary=lib11_crates_library.rlib && ./11-crates-executable

clean:
	rm -rf 11-crates-executable
	rm -rf lib11_crates_library