use std::io::prelude::*;
use std::{env, fs};

fn main() {
	// Set up 'defines.h' for our features.
	let mut defines = fs::File::open("apt-pkg-c/defines.h").unwrap();
	let mut defines_string = String::new();
	defines.read_to_string(&mut defines_string).unwrap();

	if env::var("CARGO_FEATURE_WORKER_SIZES").is_ok() {
		defines_string =
			defines_string.replace("RUST_APT_WORKER_SIZES 0", "RUST_APT_WORKER_SIZES 1");
	} else {
		defines_string =
			defines_string.replace("RUST_APT_WORKER_SIZES 1", "RUST_APT_WORKER_SIZES 0");
	}

	fs::write("apt-pkg-c/defines.h", defines_string).unwrap();

	// Set up CXX.
	let source_files = vec![
		"src/cache.rs",
		"src/progress.rs",
		"src/config.rs",
		"src/util.rs",
		"src/records.rs",
		"src/resolver.rs",
		"src/depcache.rs",
		"src/package.rs",
		"src/pkgmanager.rs",
	];

	cxx_build::bridges(source_files)
		.file("apt-pkg-c/cache.cc")
		.file("apt-pkg-c/progress.cc")
		.file("apt-pkg-c/configuration.cc")
		.file("apt-pkg-c/util.cc")
		.file("apt-pkg-c/records.cc")
		.file("apt-pkg-c/depcache.cc")
		.file("apt-pkg-c/package.cc")
		.file("apt-pkg-c/pkgmanager.cc")
		.file("apt-pkg-c/resolver.cc")
		.flag_if_supported("-std=c++14")
		.compile("rust-apt");

	// Tell Cargo about our C++ files.
	println!("cargo:rustc-link-lib=apt-pkg");
	println!("cargo:rerun-if-changed=src/cache.rs");
	println!("cargo:rerun-if-changed=src/progress.rs");
	println!("cargo:rerun-if-changed=src/config.rs");
	println!("cargo:rerun-if-changed=src/util.rs");
	println!("cargo:rerun-if-changed=src/records.rs");
	println!("cargo:rerun-if-changed=src/depcache.rs");
	println!("cargo:rerun-if-changed=src/package.rs");
	println!("cargo:rerun-if-changed=src/pkgmanager.rs");
	println!("cargo:rerun-if-changed=src/resolver.rs");

	println!("cargo:rerun-if-changed=apt-pkg-c/cache.cc");
	println!("cargo:rerun-if-changed=apt-pkg-c/cache.h");

	println!("cargo:rerun-if-changed=apt-pkg-c/progress.cc");
	println!("cargo:rerun-if-changed=apt-pkg-c/progress.h");

	println!("cargo:rerun-if-changed=apt-pkg-c/configuration.cc");
	println!("cargo:rerun-if-changed=apt-pkg-c/configuration.h");

	println!("cargo:rerun-if-changed=apt-pkg-c/util.cc");
	println!("cargo:rerun-if-changed=apt-pkg-c/util.h");

	println!("cargo:rerun-if-changed=apt-pkg-c/records.cc");
	println!("cargo:rerun-if-changed=apt-pkg-c/records.h");

	println!("cargo:rerun-if-changed=apt-pkg-c/depcache.cc");
	println!("cargo:rerun-if-changed=apt-pkg-c/depcache.h");

	println!("cargo:rerun-if-changed=apt-pkg-c/package.cc");
	println!("cargo:rerun-if-changed=apt-pkg-c/package.h");

	println!("cargo:rerun-if-changed=apt-pkg-c/pkgmanager.cc");
	println!("cargo:rerun-if-changed=apt-pkg-c/pkgmanager.h");

	println!("cargo:rerun-if-changed=apt-pkg-c/resolver.cc");
	println!("cargo:rerun-if-changed=apt-pkg-c/resolver.h");

	println!("cargo:rerun-if-changed=apt-pkg-c/defines.h");
}
