use crate::raw;
use crate::raw::apt;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::ffi;
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug)]
pub struct Package<'a> {
	// Commented attributes are to be implemented
	pub pkg_ptr: *mut apt::PkgIterator,
	_lifetime: &'a PhantomData<Cache>,
	_records: Rc<RefCell<Records>>,
	pub name: String,
	pub arch: String,
	// id: i32,
	//pub candidate: Option<Version>,
	//pub installed: Option<Version>,
	// essential: bool,
	// current_state: i32,
	// inst_state: i32,
	// selected_state: i32,
	pub has_versions: bool,
	pub has_provides: bool,
	// provides_list: List[Tuple[str, str, Version]],
	//pub _versions: Vec<Version>,
}
impl<'a> Package<'a> {
	pub fn new(
		//_cache: *mut apt::PCache,
		_records: Rc<RefCell<Records>>,
		pkg_ptr: *mut apt::PkgIterator,
		clone: bool,
	) -> Package<'a> {
		unsafe {
			Package {
				// Get a new pointer for the package
				pkg_ptr: if clone {
					apt::pkg_clone(pkg_ptr)
				} else {
					pkg_ptr
				},
				_lifetime: &PhantomData,
				_records: _records,
				name: apt::get_fullname(pkg_ptr, true),
				arch: raw::own_string(apt::pkg_arch(pkg_ptr)).unwrap(),
				has_versions: apt::pkg_has_versions(pkg_ptr),
				has_provides: apt::pkg_has_provides(pkg_ptr),
			}
		}
	}

	pub fn get_fullname(&self, pretty: bool) -> String {
		unsafe { apt::get_fullname(self.pkg_ptr, pretty) }
	}

	pub fn candidate(&self) -> Option<Version<'a>> {
		unsafe {
			let ver = apt::pkg_candidate_version(self._records.borrow_mut()._pcache, self.pkg_ptr);
			if apt::ver_end(ver) {
				return None;
			}
			Some(Version::new(Rc::clone(&self._records), ver, false))
		}
	}

	pub fn installed(&self) -> Option<Version<'a>> {
		unsafe {
			let ver = apt::pkg_current_version(self.pkg_ptr);
			if apt::ver_end(ver) {
				return None;
			}
			Some(Version::new(Rc::clone(&self._records), ver, false))
		}
	}

	pub fn is_upgradable(&self) -> bool {
		unsafe { apt::pkg_is_upgradable(self._records.borrow_mut()._pcache, self.pkg_ptr) }
	}

	/// Returns a version list starting with the newest and ending with the oldest.
	pub fn versions(&self) -> Vec<Version<'a>> {
		let mut version_map = Vec::new();
		unsafe {
			let version_iterator = apt::pkg_version_list(self.pkg_ptr);
			let mut first = true;
			loop {
				if !first {
					apt::ver_next(version_iterator);
				}
				first = false;
				if apt::ver_end(version_iterator) {
					break;
				}
				version_map.push(Version::new(
					Rc::clone(&self._records),
					version_iterator,
					true,
				));
			}
			apt::ver_release(version_iterator);
		}
		version_map
	}
}

// We must release the pointer on drop
impl<'a> Drop for Package<'a> {
	fn drop(&mut self) {
		unsafe {
			apt::pkg_release(self.pkg_ptr);
		}
	}
}

impl<'a> fmt::Display for Package<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"Package< name: {}, arch: {}, virtual: {}, provides: {}>",
			self.name, self.arch, !self.has_versions, self.has_provides
		)?;
		Ok(())
	}
}

#[derive(Debug)]
struct PackageFile {
	ver_file: *mut apt::VerFileIterator,
	index: *mut apt::PkgIndexFile,
	file: *mut apt::PkgFileIterator,
}

impl Drop for PackageFile {
	fn drop(&mut self) {
		unsafe {
			apt::ver_file_release(self.ver_file);
			apt::pkg_file_release(self.file);
			apt::pkg_index_file_release(self.index);
		}
	}
}

impl fmt::Display for PackageFile {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "package file: {:?}", self.file)?;
		Ok(())
	}
}

#[derive(Debug)]
pub struct Version<'a> {
	//_parent: RefCell<Package<'a>>,
	_lifetime: &'a PhantomData<Cache>,
	_records: Rc<RefCell<Records>>,
	pub ptr: *mut apt::VerIterator,
	//pub cache: *mut apt::PCache,
	//	pub parent_pkg: &'a Package<'a>,
	//pub name: String,
	pub version: String,
	// hash: int
	file_list: Vec<PackageFile>,
	// translated_description: Description
	// installed_size: int
	// size: int
	pub arch: String,
	// downloadable: bool
	// id: int
	pub section: String,
	pub priority: i32,
	pub priority_str: String,
	// provides_list: List[Tuple[str,str,str]]
	// depends_list: Dict[str, List[List[Dependency]]]
	// parent_pkg: Package
	// multi_arch: int
	// MULTI_ARCH_ALL: int
	// MULTI_ARCH_ALLOWED: int
	// MULTI_ARCH_ALL_ALLOWED: int
	// MULTI_ARCH_ALL_FOREIGN: int
	// MULTI_ARCH_FOREIGN: int
	// MULTI_ARCH_NO: int
	// MULTI_ARCH_NONE: int
	// MULTI_ARCH_SAME: int
}

impl<'a> Version<'a> {
	fn new(records: Rc<RefCell<Records>>, ver_ptr: *mut apt::VerIterator, clone: bool) -> Self {
		let mut package_files = Vec::new();
		unsafe {
			let ver_file = apt::ver_file(ver_ptr);
			let mut first = true;

			loop {
				if !first {
					apt::ver_file_next(ver_file);
				}

				first = false;
				if apt::ver_file_end(ver_file) {
					break;
				}
				let pkg_file = apt::ver_pkg_file(ver_file);
				package_files.push(PackageFile {
					ver_file: apt::ver_file_clone(ver_file),
					index: apt::pkg_index_file(records.borrow_mut()._pcache, pkg_file),
					file: pkg_file,
				});
			}
			apt::ver_file_release(ver_file);
			let ver_priority = apt::ver_priority(records.borrow_mut()._pcache, ver_ptr);
			Self {
				ptr: if clone {
					apt::ver_clone(ver_ptr)
				} else {
					ver_ptr
				},
				_lifetime: &PhantomData,
				priority: ver_priority,
				_records: records,
				file_list: package_files,
				version: raw::own_string(apt::ver_str(ver_ptr)).unwrap(),
				arch: raw::own_string(apt::ver_arch(ver_ptr)).unwrap(),
				section: raw::own_string(apt::ver_section(ver_ptr)).unwrap_or(String::from("None")),

				priority_str: raw::own_string(apt::ver_priority_str(ver_ptr)).unwrap(),
			}
		}
	}

	// pub fn installed() {
	// 	let ver = apt::pkg_current_version(self.pkg_ptr);
	// 	if apt::ver_end(apt::pkg_current_version(self.pkg_ptr)) { return None }
	// }

	pub fn get_uris(&self) -> Vec<String> {
		let mut uris = Vec::new();
		for package_file in &self.file_list {
			unsafe {
				let records = self._records.borrow_mut();
				records.lookup(package_file.ver_file);
				let uri = apt::ver_uri(records.ptr, package_file.index);
				if !uri.starts_with("file:") {
					uris.push(apt::ver_uri(records.ptr, package_file.index));
				}
			}
		}
		uris
	}
}

// We must release the pointer on drop
impl<'a> Drop for Version<'a> {
	fn drop(&mut self) {
		unsafe {
			apt::ver_release(self.ptr);
		}
	}
}

impl<'a> fmt::Display for Version<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"Version: <version: {}, arch: {}, section: {} Priority {} at {}>",
			self.version, self.arch, self.section, self.priority_str, self.priority,
		)?;

		Ok(())
	}
}

#[derive(Default)]
pub struct PackageSort {
	pub upgradable: bool,
	pub virtual_pkgs: bool,
}

// impl Default for PackageSort {
// 	fn default() -> PackageSort{
// 		PackageSort {
// 			iInsertMax: -1,
// 			iUpdateMax: -1,
// 			iDeleteMax: -1,
// 			iInstanceMax: -1,
// 			tFirstInstance: false,
// 			tCreateTables: false,
// 			tContinue: false,
// 		}
// 	}
// }

#[derive(Debug)]
pub struct Records {
	ptr: *mut apt::PkgRecords,
	_pcache: *mut apt::PCache,
	//pub _helper: RefCell<String>,
}

impl Records {
	pub fn new(pcache: *mut apt::PCache) -> Self {
		Records {
			ptr: unsafe { apt::pkg_records_create(pcache) },
			_pcache: pcache,
			//_helper: RefCell::new("Not been helped!".to_string()),
		}
	}

	pub fn lookup(&self, ver_file: *mut apt::VerFileIterator) {
		unsafe {
			apt::ver_file_lookup(self.ptr, ver_file);
		}
		//println!("We're helping!");
		//self._helper.replace("We've been helped!".to_string());
		//self.helped();
	}

	// pub fn helped(&self) {
	// 	//println!("{}", self._helper.borrow())
	// }
}

impl Drop for Records {
	fn drop(&mut self) {
		unsafe {
			apt::pkg_records_release(self.ptr);
		}
	}
}

#[derive(Debug)]
pub struct Cache {
	pub _cache: *mut apt::PCache,
	pointers: Vec<*mut apt::PkgIterator>,
	pub _records: Rc<RefCell<Records>>,
}

impl Drop for Cache {
	fn drop(&mut self) {
		unsafe {
			apt::pkg_cache_release(self._cache);
			for ptr in (*self.pointers).into_iter() {
				apt::pkg_release(*ptr);
			}
		}
	}
}

impl Cache {
	pub fn new() -> Self {
		unsafe {
			apt::init_config_system();
			let cache_ptr = apt::pkg_cache_create();
			Self {
				_cache: cache_ptr,
				pointers: Cache::get_pointers(apt::pkg_begin(cache_ptr)),
				_records: Rc::new(RefCell::new(Records::new(cache_ptr))),
			}
		}
	}

	pub fn clear(&mut self) {
		unsafe {
			apt::depcache_init(self._cache);
		}
	}

	pub fn validate(&self, ver: *mut apt::VerIterator) -> bool {
		unsafe { apt::validate(ver, self._cache) }
	}

	pub fn get(&self, name: &str) -> Option<Package> {
		let _name: &str;
		let _arch: &str;

		if name.contains(":") {
			let package: Vec<&str> = name.split(":").collect();

			if package.len() > 2 {
				panic!("Value is wrong");
			}

			_name = package[0];
			_arch = package[1];
		} else {
			_name = name;
			_arch = "";
		}

		let pkg_ptr = self.find_by_name(_name, _arch);
		unsafe {
			if apt::pkg_end(pkg_ptr) {
				apt::pkg_release(pkg_ptr);
				return None;
			}
		}
		Some(Package::new(Rc::clone(&self._records), pkg_ptr, false))
	}

	/// Internal method for getting a package by name
	/// Find a package by name and additionally architecture.
	/// The returned iterator will either be at the end, or at a matching package.
	fn find_by_name(&self, name: &str, arch: &str) -> *mut apt::PkgIterator {
		unsafe {
			let name = ffi::CString::new(name).unwrap();
			if !arch.is_empty() {
				let arch = ffi::CString::new(arch).unwrap();
				return apt::pkg_cache_find_name_arch(self._cache, name.as_ptr(), arch.as_ptr());
			}
			apt::pkg_cache_find_name(self._cache, name.as_ptr())
		}
	}

	pub fn sorted(&self, sort: PackageSort) -> BTreeMap<String, Package> {
		let mut package_map = BTreeMap::new();
		unsafe {
			let pkg_iterator = apt::pkg_begin(self._cache);
			let mut first = true;
			loop {
				// We have to make sure we get the first package
				if !first {
					apt::pkg_next(pkg_iterator);
				}
				first = false;
				if apt::pkg_end(pkg_iterator) {
					break;
				}

				if !sort.virtual_pkgs && !apt::pkg_has_versions(pkg_iterator) {
					continue;
				}
				if sort.upgradable && !apt::pkg_is_upgradable(self._cache, pkg_iterator) {
					continue;
				}
				package_map.insert(
					apt::get_fullname(pkg_iterator, true),
					Package::new(Rc::clone(&self._records), pkg_iterator, true),
				);
			}
			apt::pkg_release(pkg_iterator);
		}
		package_map
	}

	pub fn packages(&self) -> impl Iterator<Item = Package> + '_ {
		let pointers = &self.pointers;
		pointers
			.into_iter()
			.map(|pkg_ptr| Package::new(Rc::clone(&self._records), *pkg_ptr, true))
	}

	fn get_pointers(pkg_iterator: *mut apt::PkgIterator) -> Vec<*mut apt::PkgIterator> {
		let mut package_map = Vec::new();
		unsafe {
			let mut first = true;
			loop {
				// We have to make sure we get the first package
				if !first {
					apt::pkg_next(pkg_iterator);
				}

				first = false;
				if apt::pkg_end(pkg_iterator) {
					break;
				}
				package_map.push(apt::pkg_clone(pkg_iterator));
			}
			apt::pkg_release(pkg_iterator);
		}
		package_map
	}
}
