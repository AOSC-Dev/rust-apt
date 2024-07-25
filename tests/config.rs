mod config {
	use oma_apt::config::Config;
	use std::collections::VecDeque;
	use std::process::Command;

	#[test]
	fn clear() {
		// Test to make sure that the config populates properly.
		// Config will be empty if it hasn't been initialized.
		let config = Config::new_clear();
		config.clear_all();

		let empty_config = config.find("APT::Architecture", "");
		assert!(!config.contains("APT::Architecture"));
		assert!(empty_config.is_empty());

		// Reset the configuration which will clear and reinit.
		config.reset();

		// Now it should NOT be empty.
		let config_dump = config.find("APT::Architecture", "");
		assert!(config.contains("APT::Architecture"));
		assert!(!config_dump.is_empty());
		println!("{}", config.dump());
	}

	#[test]
	fn find_and_set() {
		let config = Config::new_clear();
		let key = "oma_apt::NotExist";

		// Find our key. It should not exist.
		assert_eq!(config.find(key, "None"), "None");

		// Set the key to something.
		config.set(key, "Exists!");

		// Find again and it should be there.
		assert_eq!(config.find(key, "None"), "Exists!");

		// Test other find functions on known defaults.
		assert!(!config.bool("APT::Install-Suggests", true));
		assert_eq!(config.int("APT::Install-Suggests", 20), 0);

		// Directory is different in CI. Just check for the name
		assert!(
			config
				.file("Dir::Cache::pkgcache", "")
				.split('/')
				.any(|x| x == "pkgcache.bin")
		);
		assert_eq!(
			config.dir("Dir::Etc::sourceparts", ""),
			"/etc/apt/sources.list.d/"
		);

		// Check if we can set a configuration list and retrieve it.
		// Make sure that the target vector is empty.
		assert!(config.find_vector("oma_apt::aptlist").is_empty());

		// Now fill our configuration vector and set it.
		let apt_list = vec!["this", "is", "my", "apt", "list"];
		config.set_vector("oma_apt::aptlist", &apt_list);

		// Retrieve a new vector from the configuration.
		let apt_vector = config.find_vector("oma_apt::aptlist");

		// If everything went smooth, our original vector should match the new one
		assert_eq!(apt_list, apt_vector);

		// Now test if we can remove a single value from the list.
		config.clear_value("oma_apt::aptlist", "my");

		// This will let us know if it worked!
		assert_eq!(
			config.find_vector("oma_apt::aptlist"),
			vec!["this", "is", "apt", "list"]
		);

		// Finally test and see if we can clear the entire list.
		config.clear("oma_apt::aptlist");
		assert!(config.find_vector("oma_apt::aptlist").is_empty());
	}

	#[test]
	fn get_architectures() {
		let config = Config::new();

		let output = dbg!(
			String::from_utf8(
				Command::new("dpkg")
					.arg("--print-architecture")
					.output()
					.unwrap()
					.stdout,
			)
			.unwrap()
		);

		let arches = dbg!(config.get_architectures());

		assert!(arches.contains(&output.strip_suffix('\n').unwrap().to_string()));
	}

	#[test]
	fn config_tree() {
		// An example of how you might walk the entire config tree.
		let config = Config::new();

		let mut stack = VecDeque::new();

		stack.push_back((config.root_tree(), 0));

		while let Some((node, indent)) = stack.pop_back() {
			let indent_str = " ".repeat(indent);

			if let Some(item) = node.sibling() {
				stack.push_back((item, indent));
			}

			if let Some(item) = node.child() {
				stack.push_back((item, indent + 2));
			}

			if let Some(tag) = node.tag() {
				if !tag.is_empty() {
					println!("{}Tag: {}", indent_str, tag);
				}
			}

			if let Some(value) = node.value() {
				if !value.is_empty() {
					println!("{}Value: {}", indent_str, value);
				}
			}
		}
	}
}
