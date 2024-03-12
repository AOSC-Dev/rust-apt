//! Contains config related structs and functions.

use crate::raw::config::raw;

/// Struct for Apt Configuration
///
/// All apt configuration methods do not require this struct.
/// You can call the bindings directly from raw::apt if you would like.
#[derive(Debug)]
pub struct Config {}

impl Default for Config {
	/// Create a new config object and safely init the config system.
	///
	/// If you initialize the struct without `new()` or `default()`
	/// You will need to manually initialize the config system.
	fn default() -> Self { Self::new() }
}

impl Config {
	/// Create a new config object and safely init the config system.
	///
	/// If you initialize the struct without `new()` or `default()`
	/// You will need to manually initialize the config system.
	pub fn new() -> Self {
		init_config_system();
		Self {}
	}

	/// Clears all configuratations, re-initialize, and returns the config
	/// object.
	pub fn new_clear() -> Self {
		raw::config_clear_all();
		Self::new()
	}

	/// Resets the configurations.
	///
	/// If you'd like to clear everything and NOT reinit
	/// you can call `self.clear_all` or `raw::config_clear_all` directly
	pub fn reset(&self) {
		self.clear_all();
		init_config_system();
	}

	/// Clears all values from a key.
	///
	/// If the value is a list, the entire list is cleared.
	/// If you need to clear 1 value from a list see `self.clear_value`
	pub fn clear(&self, key: &str) { raw::config_clear(key.to_string()); }

	/// Clear a single value from a list.
	/// Used for removing one item in an apt configuruation list
	pub fn clear_value(&self, key: &str, value: &str) {
		raw::config_clear_value(key.to_string(), value.to_string());
	}

	/// Clears all configuratations.
	///
	/// This will leave you with an empty configuration object
	/// and most things probably won't work right.
	pub fn clear_all(&self) { raw::config_clear_all(); }

	/// Returns a string dump of configuration options separated by `\n`
	pub fn dump(&self) -> String { raw::config_dump() }

	/// Find a key and return it's value as a string.
	///
	/// default is what will be returned if nothing is found.
	pub fn find(&self, key: &str, default: &str) -> String {
		raw::config_find(key.to_string(), default.to_string())
	}

	/// Exactly like find but takes no default and returns an option instead.
	pub fn get(&self, key: &str) -> Option<String> {
		let value = raw::config_find(key.to_string(), "".to_string());
		if value.is_empty() {
			return None;
		}
		Some(value)
	}

	/// Find a file and return it's value as a string.
	///
	/// default is what will be returned if nothing is found.
	///
	/// `key = "Dir::Cache::pkgcache"` should return
	/// `/var/cache/apt/pkgcache.bin`
	///
	/// There is not much difference in `self.dir` and `self.file`
	///
	/// `dir` will return with a trailing `/` where `file` will not.
	pub fn file(&self, key: &str, default: &str) -> String {
		raw::config_find_file(key.to_string(), default.to_string())
	}

	/// Find a directory and return it's value as a string.
	///
	/// default is what will be returned if nothing is found.
	///
	/// `key = "Dir::Etc::sourceparts"` should return `/etc/apt/sources.list.d/`
	///
	/// There is not much difference in `self.dir` and `self.file`
	///
	/// `dir` will return with a trailing `/` where `file` will not.
	pub fn dir(&self, key: &str, default: &str) -> String {
		raw::config_find_dir(key.to_string(), default.to_string())
	}

	/// Same as find, but for boolean values.
	pub fn bool(&self, key: &str, default: bool) -> bool {
		raw::config_find_bool(key.to_string(), default)
	}

	/// Same as find, but for i32 values.
	pub fn int(&self, key: &str, default: i32) -> i32 {
		raw::config_find_int(key.to_string(), default)
	}

	/// Return a vector for an Apt configuration list.
	///
	/// An example of a common key that contains a list `raw::NeverAutoRemove`.
	pub fn find_vector(&self, key: &str) -> Vec<String> { raw::config_find_vector(key.to_string()) }

	/// Return a vector of supported architectures on this system.
	/// The main architecture is the first in the list.
	pub fn get_architectures(&self) -> Vec<String> { raw::config_get_architectures() }

	/// Simply check if a key exists.
	pub fn contains(&self, key: &str) -> bool { raw::config_exists(key.to_string()) }

	/// Set the given key to the specified value.
	pub fn set(&self, key: &str, value: &str) {
		raw::config_set(key.to_string(), value.to_string())
	}

	/// Add strings from a vector into an apt configuration list.
	///
	/// If the configuration key is not a list,
	/// you will receive a vector with one item.
	///
	/// Example:
	/// ```
	/// use rust_apt::config::Config;
	/// let config = Config::new();
	///
	/// let apt_list = vec!["This", "is", "my", "apt", "list"];
	/// // Using "AptList" here will not work and will panic.
	/// config.set_vector("AptList", &apt_list);
	/// ```
	pub fn set_vector(&self, key: &str, values: &Vec<&str>) {
		let mut vec_key = String::from(key);
		if !vec_key.ends_with("::") {
			vec_key.push_str("::");
		}

		for value in values {
			raw::config_set(vec_key.to_string(), value.to_string());
		}
	}
}

/// Safely Init Apt Configuration and System.
///
/// If the configuration has already been initialized, don't reinit.
///
/// This could cause some things to get reset.
pub fn init_config_system() {
	if raw::config_find("APT".to_string(), "".to_string()).is_empty() {
		raw::init_config();
	}
	raw::init_system();
}
