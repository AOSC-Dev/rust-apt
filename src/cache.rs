use std::cell::RefCell;
use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;

use once_cell::unsync::OnceCell;

#[deny(clippy::not_unsafe_ptr_arg_deref)]
use crate::raw::apt;

#[derive(Debug)]
pub struct Package<'a> {
	// Commented fields are to be implemented
	_lifetime: &'a PhantomData<Cache>,
	records: Rc<RefCell<Records>>,
	depcache: Rc<RefCell<DepCache>>,
	ptr: *mut apt::PkgIterator,
	pub name: String,
	pub arch: String,
	pub id: i32,
	pub essential: bool,
	pub current_state: i32,
	pub inst_state: i32,
	pub selected_state: i32,
	pub has_versions: bool,
	pub has_provides: bool,
	// provides_list: List[Tuple[str, str, Version]],
}

impl<'a> Package<'a> {
	pub fn new(
		records: Rc<RefCell<Records>>,
		depcache: Rc<RefCell<DepCache>>,
		pkg_ptr: *mut apt::PkgIterator,
	) -> Package<'a> {
		unsafe {
			Package {
				_lifetime: &PhantomData,
				ptr: pkg_ptr,
				records,
				depcache,
				name: apt::get_fullname(pkg_ptr, true),
				arch: apt::pkg_arch(pkg_ptr),
				id: apt::pkg_id(pkg_ptr),
				essential: apt::pkg_essential(pkg_ptr),
				current_state: apt::pkg_current_state(pkg_ptr),
				inst_state: apt::pkg_inst_state(pkg_ptr),
				selected_state: apt::pkg_selected_state(pkg_ptr),
				has_versions: apt::pkg_has_versions(pkg_ptr),
				has_provides: apt::pkg_has_provides(pkg_ptr),
			}
		}
	}

	/// Get the fullname of the package.
	///
	/// Pretty is a bool that will omit the native arch.
	///
	/// For example on an amd64 system:
	///
	/// `pkg.get_fullname(true)` would return just `"apt"` for the amd64 package
	/// and `"apt:i386"` for the i386 package.
	///
	/// `pkg.get_fullname(false)` would return `"apt:amd64"` for the amd64
	/// version and `"apt:i386"` for the i386 package.
	pub fn get_fullname(&self, pretty: bool) -> String {
		unsafe { apt::get_fullname(self.ptr, pretty) }
	}

	/// Returns the version object of the candidate.
	///
	/// If there isn't a candidate, returns None
	pub fn candidate(&self) -> Option<Version<'a>> {
		unsafe {
			let ver = apt::pkg_candidate_version(self.records.borrow_mut().pcache, self.ptr);
			if apt::ver_end(ver) {
				return None;
			}
			Some(Version::new(Rc::clone(&self.records), ver, false))
		}
	}

	/// Returns the version object of the installed version.
	///
	/// If there isn't an installed version, returns None
	pub fn installed(&self) -> Option<Version<'a>> {
		unsafe {
			let ver = apt::pkg_current_version(self.ptr);
			if apt::ver_end(ver) {
				return None;
			}
			Some(Version::new(Rc::clone(&self.records), ver, false))
		}
	}

	/// Check if the package is installed.
	pub fn is_installed(&self) -> bool { unsafe { apt::pkg_is_installed(self.ptr) } }

	/// Check if the package is upgradable.
	pub fn is_upgradable(&self) -> bool { self.depcache.borrow().is_upgradable(self.ptr) }

	/// Check if the package is auto installed. (Not installed by the user)
	pub fn is_auto_installed(&self) -> bool { self.depcache.borrow().is_auto_installed(self.ptr) }

	/// Check if the package is auto removable
	pub fn is_auto_removable(&self) -> bool { self.depcache.borrow().is_auto_removable(self.ptr) }

	/// Check if the package is now broken
	pub fn is_now_broken(&self) -> bool { self.depcache.borrow().is_now_broken(self.ptr) }

	/// Check if the package package installed is broken
	pub fn is_inst_broken(&self) -> bool { self.depcache.borrow().is_inst_broken(self.ptr) }

	/// Check if the package is marked install
	pub fn marked_install(&self) -> bool { self.depcache.borrow().marked_install(self.ptr) }

	/// Check if the package is marked upgrade
	pub fn marked_upgrade(&self) -> bool { self.depcache.borrow().marked_upgrade(self.ptr) }

	/// Check if the package is marked delete
	pub fn marked_delete(&self) -> bool { self.depcache.borrow().marked_delete(self.ptr) }

	/// Check if the package is marked keep
	pub fn marked_keep(&self) -> bool { self.depcache.borrow().marked_keep(self.ptr) }

	/// Check if the package is marked downgrade
	pub fn marked_downgrade(&self) -> bool { self.depcache.borrow().marked_downgrade(self.ptr) }

	/// Check if the package is marked reinstall
	pub fn marked_reinstall(&self) -> bool { self.depcache.borrow().marked_reinstall(self.ptr) }

	/// Returns a version list starting with the newest and ending with the
	/// oldest.
	pub fn versions(&self) -> Vec<Version<'a>> {
		let mut version_map = Vec::new();
		unsafe {
			let version_iterator = apt::pkg_version_list(self.ptr);
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
					Rc::clone(&self.records),
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
			apt::pkg_release(self.ptr);
		}
	}
}

impl<'a> fmt::Display for Package<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"Package< name: {}, arch: {}, id: {}, essential: {}, states: [curr: {}, inst {}, sel \
			 {}], virtual: {}, provides: {}>",
			self.name,
			self.arch,
			self.id,
			self.essential,
			self.current_state,
			self.inst_state,
			self.selected_state,
			!self.has_versions,
			self.has_provides
		)?;
		Ok(())
	}
}

#[derive(Debug)]
struct PackageFile {
	ver_file: *mut apt::VerFileIterator,
	pkg_file: *mut apt::PkgFileIterator,
	index: *mut apt::PkgIndexFile,
}

impl Drop for PackageFile {
	fn drop(&mut self) {
		unsafe {
			apt::ver_file_release(self.ver_file);
			apt::pkg_file_release(self.pkg_file);
			apt::pkg_index_file_release(self.index);
		}
	}
}

impl fmt::Display for PackageFile {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "package file: {:?}", self.pkg_file)?;
		Ok(())
	}
}

#[derive(Debug)]
pub struct Version<'a> {
	//_parent: RefCell<Package<'a>>,
	_lifetime: &'a PhantomData<Cache>,
	_records: Rc<RefCell<Records>>,
	desc_ptr: *mut apt::DescIterator,
	ptr: *mut apt::VerIterator,
	file_list: OnceCell<Vec<PackageFile>>,
	pub pkgname: String,
	pub version: String,
	pub size: i32,
	pub installed_size: i32,
	pub arch: String,
	pub downloadable: bool,
	pub id: i32,
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
		unsafe {
			let ver_priority = apt::ver_priority(records.borrow_mut().pcache, ver_ptr);
			Self {
				ptr: if clone { apt::ver_clone(ver_ptr) } else { ver_ptr },
				desc_ptr: apt::ver_desc_file(ver_ptr),
				_records: records,
				_lifetime: &PhantomData,
				pkgname: apt::ver_name(ver_ptr),
				priority: ver_priority,
				file_list: OnceCell::new(),
				version: apt::ver_str(ver_ptr),
				size: apt::ver_size(ver_ptr),
				installed_size: apt::ver_installed_size(ver_ptr),
				arch: apt::ver_arch(ver_ptr),
				downloadable: apt::ver_downloadable(ver_ptr),
				id: apt::ver_id(ver_ptr),
				section: apt::ver_section(ver_ptr),
				priority_str: apt::ver_priority_str(ver_ptr),
			}
		}
	}

	/// Internal Method for Generating the PackageFiles
	fn gen_file_list(&self) -> Vec<PackageFile> {
		let mut package_files = Vec::new();
		unsafe {
			let ver_file = apt::ver_file(self.ptr);
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
					pkg_file,
					index: apt::pkg_index_file(self._records.borrow_mut().pcache, pkg_file),
				});
			}
			apt::ver_file_release(ver_file);
		}
		package_files
	}

	/// Check if the version is installed
	pub fn is_installed(&self) -> bool { unsafe { apt::ver_installed(self.ptr) } }

	/// Get the translated long description
	pub fn description(&self) -> String {
		let records = self._records.borrow_mut();
		records.lookup(Lookup::Desc(self.desc_ptr));
		records.description()
	}

	/// Get the translated short description
	pub fn summary(&self) -> String {
		let records = self._records.borrow_mut();
		records.lookup(Lookup::Desc(self.desc_ptr));
		records.summary()
	}

	/// Get the sha256 hash. If there isn't one returns None
	/// This is equivalent to `version.hash("sha256")`
	pub fn sha256(&self) -> Option<String> { self.hash("sha256") }

	/// Get the sha512 hash. If there isn't one returns None
	/// This is equivalent to `version.hash("sha512")`
	pub fn sha512(&self) -> Option<String> { self.hash("sha512") }

	/// Get the hash specified. If there isn't one returns None
	/// `version.hash("md5sum")`
	pub fn hash(&self, hash_type: &str) -> Option<String> {
		let package_files = self.file_list.get_or_init(|| self.gen_file_list());

		if let Some(pkg_file) = package_files.into_iter().next() {
			let records = self._records.borrow_mut();
			records.lookup(Lookup::VerFile(pkg_file.ver_file));
			return records.hash_find(hash_type);
		}
		None
	}

	/// Returns an iterator of URIs for the version
	pub fn uris(&'a self) -> impl Iterator<Item = String> + 'a {
		self.file_list
			.get_or_init(|| self.gen_file_list())
			.into_iter()
			.filter_map(|package_file| {
				let records = self._records.borrow_mut();
				records.lookup(Lookup::VerFile(package_file.ver_file));

				let uri = unsafe { apt::ver_uri(records.ptr, package_file.index) };
				if !uri.starts_with("file:") {
					Some(uri)
				} else {
					None
				}
			})
	}
}

// We must release the pointer on drop
impl<'a> Drop for Version<'a> {
	fn drop(&mut self) {
		unsafe {
			apt::ver_release(self.ptr);
			apt::ver_desc_release(self.desc_ptr)
		}
	}
}

impl<'a> fmt::Display for Version<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{}: Version {} <ID: {}, arch: {}, size: {}, installed_size: {}, section: {} Priority \
			 {} at {}, downloadable: {}>",
			self.pkgname,
			self.version,
			self.id,
			self.arch,
			unit_str(self.size),
			unit_str(self.installed_size),
			self.section,
			self.priority_str,
			self.priority,
			self.downloadable,
		)?;
		Ok(())
	}
}

#[derive(Debug, Default, PartialEq)]
pub struct PackageSort {
	pub upgradable: bool,
	pub virtual_pkgs: bool,
	pub installed: bool,
	pub auto_installed: bool,
	pub auto_removable: bool,
}

impl PackageSort {
	/// If true, only packages that are upgradable will be included
	pub fn upgradable(mut self, switch: bool) -> Self {
		self.upgradable = switch;
		self
	}

	/// If true, virtual pkgs will be included
	pub fn virtual_pkgs(mut self, switch: bool) -> Self {
		self.virtual_pkgs = switch;
		self
	}

	/// If true, only packages that are installed will be included
	pub fn installed(mut self, switch: bool) -> Self {
		self.installed = switch;
		self
	}

	/// If true, only packages that are auto installed will be included
	pub fn auto_installed(mut self, switch: bool) -> Self {
		self.auto_installed = switch;
		self
	}

	/// If true, only packages that are auto removable will be included
	pub fn auto_removable(mut self, switch: bool) -> Self {
		self.auto_removable = switch;
		self
	}
}

#[derive(Debug, PartialEq)]
pub enum Lookup {
	Desc(*mut apt::DescIterator),
	VerFile(*mut apt::VerFileIterator),
}

#[derive(Debug)]
pub struct Records {
	ptr: *mut apt::PkgRecords,
	pcache: *mut apt::PCache,
	last: RefCell<Option<Lookup>>,
}

impl Records {
	pub fn new(pcache: *mut apt::PCache) -> Self {
		Records {
			ptr: unsafe { apt::pkg_records_create(pcache) },
			pcache,
			last: RefCell::new(None),
		}
	}

	pub fn lookup(&self, record: Lookup) {
		// Check if what we're looking up is currently looked up.
		if let Some(last) = self.last.borrow().as_ref() {
			if last == &record {
				return;
			}
		}

		// Call the correct binding depending on what we're looking up.
		unsafe {
			match &record {
				Lookup::Desc(desc) => {
					apt::desc_file_lookup(self.ptr, *desc);
				},
				Lookup::VerFile(ver_file) => {
					apt::ver_file_lookup(self.ptr, *ver_file);
				},
			}
		}
		// Finally replace the stored value for the next lookup
		self.last.replace(Some(record));
	}

	pub fn description(&self) -> String { unsafe { apt::long_desc(self.ptr) } }

	pub fn summary(&self) -> String { unsafe { apt::short_desc(self.ptr) } }

	pub fn hash_find(&self, hash_type: &str) -> Option<String> {
		unsafe {
			let hash = apt::hash_find(self.ptr, hash_type.to_string());
			if hash == "KeyError" {
				return None;
			} else {
				return Some(hash);
			}
		}
	}
}

impl Drop for Records {
	fn drop(&mut self) {
		unsafe {
			apt::pkg_records_release(self.ptr);
		}
	}
}

/// Internal Struct for managing apt's pkgDepCache.
#[derive(Debug)]
pub struct DepCache {
	ptr: OnceCell<*mut apt::PkgDepCache>,
	pcache: *mut apt::PCache,
}

// DepCache does not have a drop because we don't need to free the pointer.
// The pointer is freed when the cache is dropped
// DepCache is not initialized with the cache as it slows down some operations
// Instead we have this struct to lazily initialize when we need it.
impl DepCache {
	pub fn new(pcache: *mut apt::PCache) -> Self {
		DepCache {
			ptr: OnceCell::new(),
			pcache,
		}
	}

	/// Internal helper to init the depcache if it hasn't been already.
	fn ptr(&self) -> *mut apt::PkgDepCache {
		*self
			.ptr
			.get_or_init(|| unsafe { apt::depcache_create(self.pcache) })
	}

	pub fn clear(&self) {
		unsafe {
			apt::depcache_create(self.pcache);
		}
	}

	pub fn is_upgradable(&self, pkg_ptr: *mut apt::PkgIterator) -> bool {
		unsafe { apt::pkg_is_upgradable(self.ptr(), pkg_ptr) }
	}

	pub fn is_auto_installed(&self, pkg_ptr: *mut apt::PkgIterator) -> bool {
		unsafe { apt::pkg_is_auto_installed(self.ptr(), pkg_ptr) }
	}

	pub fn is_auto_removable(&self, pkg_ptr: *mut apt::PkgIterator) -> bool {
		let dep_ptr = self.ptr();
		unsafe {
			(apt::pkg_is_installed(pkg_ptr) || apt::pkg_marked_install(dep_ptr, pkg_ptr))
				&& apt::pkg_is_garbage(self.ptr(), pkg_ptr)
		}
	}

	pub fn marked_install(&self, pkg_ptr: *mut apt::PkgIterator) -> bool {
		unsafe { apt::pkg_marked_install(self.ptr(), pkg_ptr) }
	}

	pub fn marked_upgrade(&self, pkg_ptr: *mut apt::PkgIterator) -> bool {
		unsafe { apt::pkg_marked_upgrade(self.ptr(), pkg_ptr) }
	}

	pub fn marked_delete(&self, pkg_ptr: *mut apt::PkgIterator) -> bool {
		unsafe { apt::pkg_marked_delete(self.ptr(), pkg_ptr) }
	}

	pub fn marked_keep(&self, pkg_ptr: *mut apt::PkgIterator) -> bool {
		unsafe { apt::pkg_marked_keep(self.ptr(), pkg_ptr) }
	}

	pub fn marked_downgrade(&self, pkg_ptr: *mut apt::PkgIterator) -> bool {
		unsafe { apt::pkg_marked_downgrade(self.ptr(), pkg_ptr) }
	}

	pub fn marked_reinstall(&self, pkg_ptr: *mut apt::PkgIterator) -> bool {
		unsafe { apt::pkg_marked_reinstall(self.ptr(), pkg_ptr) }
	}

	pub fn is_now_broken(&self, pkg_ptr: *mut apt::PkgIterator) -> bool {
		unsafe { apt::pkg_is_now_broken(self.ptr(), pkg_ptr) }
	}

	pub fn is_inst_broken(&self, pkg_ptr: *mut apt::PkgIterator) -> bool {
		unsafe { apt::pkg_is_inst_broken(self.ptr(), pkg_ptr) }
	}
}

#[derive(Debug)]
pub struct Cache {
	pub ptr: *mut apt::PCache,
	pub records: Rc<RefCell<Records>>,
	depcache: Rc<RefCell<DepCache>>,
}

impl Drop for Cache {
	fn drop(&mut self) {
		unsafe {
			apt::pkg_cache_release(self.ptr);
		}
	}
}

impl Default for Cache {
	fn default() -> Self { Self::new() }
}

impl Cache {
	/// Initialize the configuration system, open and return the cache.
	///
	/// This is the entry point for all operations of this crate.
	pub fn new() -> Self {
		apt::init_config_system();
		let cache_ptr = apt::pkg_cache_create();
		Self {
			ptr: cache_ptr,
			records: Rc::new(RefCell::new(Records::new(cache_ptr))),
			depcache: Rc::new(RefCell::new(DepCache::new(cache_ptr))),
		}
	}

	/// Clears all changes made to packages.
	///
	/// Currently this doesn't do anything as we can't manipulate packages.
	pub fn clear(&self) { self.depcache.borrow().clear(); }

	/// Returns an iterator of SourceURIs.
	///
	/// These are the files that `apt update` will fetch.
	pub fn sources(&self) -> impl Iterator<Item = apt::SourceFile> + '_ {
		unsafe { apt::source_uris(self.ptr).into_iter() }
	}

	// Disabled as it doesn't really work yet. Would likely need to
	// Be on the objects them self and not the cache
	// pub fn validate(&self, ver: *mut apt::VerIterator) -> bool {
	// 	unsafe { apt::validate(ver, self._cache) }
	// }

	/// Get a single package.
	///
	/// `cache.get("apt")` Returns a Package object for the native arch.
	///
	/// `cache.get("apt:i386")` Returns a Package object for the i386 arch
	pub fn get<'a>(&'a self, name: &str) -> Option<Package<'a>> {
		let mut fields = name.split(':');

		let name = fields.next()?;
		let arch = fields.next().unwrap_or_default();
		let pkg_ptr = self.find_by_name(name, arch);

		unsafe {
			if apt::pkg_end(pkg_ptr) {
				apt::pkg_release(pkg_ptr);
				return None;
			}
		}
		Some(Package::new(
			Rc::clone(&self.records),
			Rc::clone(&self.depcache),
			pkg_ptr,
		))
	}

	/// Internal method for getting a package by name
	///
	/// Find a package by name and additionally architecture.
	///
	/// The returned iterator will either be at the end, or at a matching
	/// package.
	fn find_by_name(&self, name: &str, arch: &str) -> *mut apt::PkgIterator {
		unsafe {
			if !arch.is_empty() {
				return apt::pkg_cache_find_name_arch(self.ptr, name, arch);
			}
			apt::pkg_cache_find_name(self.ptr, name)
		}
	}

	/// An iterator of packages sorted by package name.
	///
	/// Slower than the `cache.packages` method.
	pub fn sorted<'a>(&'a self, sort: &'a PackageSort) -> impl Iterator<Item = Package> + '_ {
		let mut pkgs = self.packages(sort).collect::<Vec<Package>>();
		pkgs.sort_by_cached_key(|pkg| pkg.name.to_owned());
		pkgs.into_iter()
	}

	/// An iterator of packages not sorted by name.
	///
	/// Faster than the `cache.sorted` method.
	pub fn packages<'a>(&'a self, sort: &'a PackageSort) -> impl Iterator<Item = Package> + '_ {
		Self::pointers(unsafe { apt::pkg_begin(self.ptr) })
			.filter_map(move |pkg_ptr| self.sort_package(pkg_ptr, sort))
	}

	/// Internal method for sorting packages.
	fn sort_package(&self, pkg_ptr: *mut apt::PkgIterator, sort: &PackageSort) -> Option<Package> {
		unsafe {
			if (!sort.virtual_pkgs && !apt::pkg_has_versions(pkg_ptr))
				|| (sort.upgradable && !self.depcache.borrow().is_upgradable(pkg_ptr))
				|| (sort.installed && !apt::pkg_is_installed(pkg_ptr))
				|| (sort.auto_installed && !self.depcache.borrow().is_auto_installed(pkg_ptr))
				|| (sort.auto_removable && !self.depcache.borrow().is_auto_removable(pkg_ptr))
			{
				apt::pkg_release(pkg_ptr);
				return None;
			}
		}
		Some(Package::new(
			Rc::clone(&self.records),
			Rc::clone(&self.depcache),
			pkg_ptr,
		))
	}

	/// Internal method for iterating apt's package pointers.
	fn pointers(iter_ptr: *mut apt::PkgIterator) -> impl Iterator<Item = *mut apt::PkgIterator> {
		unsafe {
			std::iter::from_fn(move || {
				if apt::pkg_end(iter_ptr) {
					apt::pkg_release(iter_ptr);
					return None;
				}

				let current = apt::pkg_clone(iter_ptr);
				apt::pkg_next(iter_ptr);
				Some(current)
			})
		}
	}
}

/// Converts a version's size into human readable output.
///
/// `println!("{}", unit_str(version.size))`
pub fn unit_str(val: i32) -> String {
	let num: i32 = 1000;
	if val > num.pow(2) {
		return format!("{:.2} MB", val as f32 / 1000.0 / 1000.0);
	} else if val > num {
		return format!("{:.2} kB", val as f32 / 1000.0);
	}
	return format!("{val} B");
}
