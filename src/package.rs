//! Contains Package, Version and Dependency Structs.
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;

use once_cell::unsync::OnceCell;

use crate::cache::{unit_str, Cache, DepCache, NumSys, Records};
use crate::raw::apt;

/// A struct representing an `apt` Package
#[derive(Debug)]
pub struct Package<'a> {
	_lifetime: &'a PhantomData<Cache>,
	records: Rc<RefCell<Records>>,
	depcache: Rc<RefCell<DepCache>>,
	pub(crate) ptr: apt::PackagePtr,
}

impl<'a> Package<'a> {
	pub fn new(
		records: Rc<RefCell<Records>>,
		depcache: Rc<RefCell<DepCache>>,
		pkg_ptr: apt::PackagePtr,
	) -> Package<'a> {
		Package {
			_lifetime: &PhantomData,
			records,
			depcache,
			ptr: pkg_ptr,
		}
	}

	/// Get the name of the package.
	/// If the arch is not native this will return "name:arch".
	pub fn name(&self) -> String { apt::get_fullname(&self.ptr, true) }

	/// Get the architecture of the package.
	pub fn arch(&self) -> String { apt::pkg_arch(&self.ptr) }

	/// Get the ID of the package.
	pub fn id(&self) -> u32 { apt::pkg_id(&self.ptr) }

	/// The current state of the package.
	pub fn current_state(&self) -> u8 { apt::pkg_current_state(&self.ptr) }

	/// The installed state of the package.
	pub fn inst_state(&self) -> u8 { apt::pkg_inst_state(&self.ptr) }

	/// The selected state of the package.
	pub fn selected_state(&self) -> u8 { apt::pkg_selected_state(&self.ptr) }

	/// Check if the package is essnetial or not.
	pub fn essential(&self) -> bool { apt::pkg_essential(&self.ptr) }

	/// Check if the package has versions.
	pub fn has_versions(&self) -> bool { apt::pkg_has_versions(&self.ptr) }

	/// Check if the package has provides.
	pub fn has_provides(&self) -> bool { apt::pkg_has_provides(&self.ptr) }

	/// Get the fullname of the package.
	///
	/// Pretty is a bool that will omit the native arch.
	///
	/// For example on an amd64 system:
	///
	/// ```
	/// use rust_apt::cache::Cache;
	/// let cache = Cache::new();
	/// if let Some(pkg) = cache.get("apt") {
	///    // Prints just "apt"
	///    println!("{}", pkg.fullname(true));
	///    // Prints "apt:amd64"
	///    println!("{}", pkg.fullname(false));
	/// };
	///
	/// if let Some(pkg) = cache.get("apt:i386") {
	///    // Prints "apt:i386" for the i386 package
	///    println!("{}", pkg.fullname(true));
	/// };
	/// ```
	pub fn fullname(&self, pretty: bool) -> String { apt::get_fullname(&self.ptr, pretty) }

	/// Returns the version object of the candidate.
	///
	/// If there isn't a candidate, returns None
	pub fn candidate(&self) -> Option<Version<'a>> {
		let ver = apt::pkg_candidate_version(&self.records.borrow().cache.borrow(), &self.ptr);
		if ver.ptr.is_null() {
			return None;
		}
		Some(Version::new(Rc::clone(&self.records), ver))
	}

	/// Returns the version object of the installed version.
	///
	/// If there isn't an installed version, returns None
	pub fn installed(&self) -> Option<Version<'a>> {
		let ver = apt::pkg_current_version(&self.ptr);
		if ver.ptr.is_null() {
			return None;
		}
		Some(Version::new(Rc::clone(&self.records), ver))
	}

	/// Check if the package is installed.
	pub fn is_installed(&self) -> bool { apt::pkg_is_installed(&self.ptr) }

	/// Check if the package is upgradable.
	///
	/// `skip_depcache = true` increases performance by skipping the pkgDepCache
	/// Skipping the depcache is very unnecessary if it's already been
	/// initialized If you're not sure, set `skip_depcache = false`
	pub fn is_upgradable(&self, skip_depcache: bool) -> bool {
		self.depcache
			.borrow()
			.is_upgradable(&self.ptr, skip_depcache)
	}

	/// Check if the package is auto installed. (Not installed by the user)
	pub fn is_auto_installed(&self) -> bool { self.depcache.borrow().is_auto_installed(&self.ptr) }

	/// Check if the package is auto removable
	pub fn is_auto_removable(&self) -> bool { self.depcache.borrow().is_auto_removable(&self.ptr) }

	/// Check if the package is now broken
	pub fn is_now_broken(&self) -> bool { self.depcache.borrow().is_now_broken(&self.ptr) }

	/// Check if the package package installed is broken
	pub fn is_inst_broken(&self) -> bool { self.depcache.borrow().is_inst_broken(&self.ptr) }

	/// Check if the package is marked install
	pub fn marked_install(&self) -> bool { self.depcache.borrow().marked_install(&self.ptr) }

	/// Check if the package is marked upgrade
	pub fn marked_upgrade(&self) -> bool { self.depcache.borrow().marked_upgrade(&self.ptr) }

	/// Check if the package is marked delete
	pub fn marked_delete(&self) -> bool { self.depcache.borrow().marked_delete(&self.ptr) }

	/// Check if the package is marked keep
	pub fn marked_keep(&self) -> bool { self.depcache.borrow().marked_keep(&self.ptr) }

	/// Check if the package is marked downgrade
	pub fn marked_downgrade(&self) -> bool { self.depcache.borrow().marked_downgrade(&self.ptr) }

	/// Check if the package is marked reinstall
	pub fn marked_reinstall(&self) -> bool { self.depcache.borrow().marked_reinstall(&self.ptr) }

	/// Returns a version list starting with the newest and ending with the
	/// oldest.
	pub fn versions(&self) -> impl Iterator<Item = Version<'a>> + '_ {
		apt::pkg_version_list(&self.ptr)
			.into_iter()
			.map(|ver_ptr| Version::new(Rc::clone(&self.records), ver_ptr))
	}
}

impl<'a> fmt::Display for Package<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"Package< name: {}, arch: {}, id: {}, essential: {}, states: [curr: {}, inst {}, sel \
			 {}], virtual: {}, provides: {}>",
			self.name(),
			self.arch(),
			self.id(),
			self.essential(),
			self.current_state(),
			self.inst_state(),
			self.selected_state(),
			!self.has_versions(),
			self.has_provides(),
		)?;
		Ok(())
	}
}

/// A struct representing a version of an `apt` Package
#[derive(Debug)]
pub struct Version<'a> {
	_lifetime: &'a PhantomData<Cache>,
	ptr: apt::VersionPtr,
	records: Rc<RefCell<Records>>,
	file_list: OnceCell<Vec<apt::PackageFile>>,
	depends_list: OnceCell<HashMap<String, Vec<Dependency>>>,
}

impl<'a> Version<'a> {
	fn new(records: Rc<RefCell<Records>>, ver_ptr: apt::VersionPtr) -> Self {
		Self {
			_lifetime: &PhantomData,
			records,
			file_list: OnceCell::new(),
			depends_list: OnceCell::new(),
			ptr: ver_ptr,
		}
	}

	/// The name of the versions Parent Package.
	pub fn pkgname(&self) -> String { apt::ver_name(&self.ptr) }

	/// The architecture of the version.
	pub fn arch(&self) -> String { apt::ver_arch(&self.ptr) }

	/// The version string of the version. "1.4.10"
	pub fn version(&self) -> String { apt::ver_str(&self.ptr) }

	/// The section of the version as shown in `apt show`.
	pub fn section(&self) -> String { apt::ver_section(&self.ptr) }

	/// The priority string as shown in `apt show`.
	pub fn priority_str(&self) -> String { apt::ver_priority_str(&self.ptr) }

	/// The name of the source package the version was built from.
	pub fn source_name(&self) -> String { apt::ver_source_name(&self.ptr) }

	/// The version of the source package.
	pub fn source_version(&self) -> String { apt::ver_source_version(&self.ptr) }

	/// The priority of the package as shown in `apt policy`.
	pub fn priority(&self) -> i32 {
		apt::ver_priority(&self.records.borrow().cache.borrow(), &self.ptr)
	}

	/// The size of the .deb file.
	pub fn size(&self) -> u64 { apt::ver_size(&self.ptr) }

	/// The uncompressed size of the .deb file.
	pub fn installed_size(&self) -> u64 { apt::ver_installed_size(&self.ptr) }

	/// The ID of the version.
	pub fn id(&self) -> u32 { apt::ver_id(&self.ptr) }

	/// If the version is able to be downloaded.
	pub fn downloadable(&self) -> bool { apt::ver_downloadable(&self.ptr) }

	/// Check if the version is installed
	pub fn is_installed(&self) -> bool { apt::ver_installed(&self.ptr) }

	/// Internal Method for Generating the PackageFiles
	fn gen_file_list(&self) -> Vec<apt::PackageFile> {
		apt::pkg_file_list(&self.records.borrow().cache.borrow(), &self.ptr)
	}

	/// Returns a reference to the Dependency Map owned by the Version
	/// ```
	/// let keys = [
	///    "Depends",
	///    "PreDepends",
	///    "Suggests",
	///    "Recommends",
	///    "Conflicts",
	///    "Replaces",
	///    "Obsoletes",
	///    "Breaks",
	///    "Enhances",
	/// ];
	/// ```
	/// Dependencies are in a `Vec<Dependency>`
	///
	/// The Dependency struct represents an Or Group of dependencies.
	/// The base deps are located in `Dependency.base_deps`
	///
	/// For example where we use the `"Depends"` key:
	///
	/// ```
	/// use rust_apt::cache::Cache;
	/// let cache = Cache::new();
	/// let version = cache.get("apt").unwrap().candidate().unwrap();
	/// for dep in version.depends_map().get("Depends").unwrap() {
	///    if dep.is_or() {
	///        for base_dep in &dep.base_deps {
	///            println!("{}", base_dep.name())
	///        }
	///    } else {
	///        // is_or is false so there is only one BaseDep
	///        println!("{}", dep.first().name())
	///    }
	/// }
	/// ```
	pub fn depends_map(&self) -> &HashMap<String, Vec<Dependency>> {
		self.depends_list.get_or_init(|| self.gen_depends())
	}

	/// Returns a reference Vector, if it exists, for the given key.
	///
	/// See the doc for `depends_map()` for more information.
	pub fn get_depends(&self, key: &str) -> Option<&Vec<Dependency>> {
		self.depends_list
			.get_or_init(|| self.gen_depends())
			.get(key)
	}

	/// Returns a Reference Vector, if it exists, for "Enhances".
	pub fn enhances(&self) -> Option<&Vec<Dependency>> { self.get_depends("Enhances") }

	/// Returns a Reference Vector, if it exists, for "Depends" and
	/// "PreDepends".
	pub fn dependencies(&self) -> Option<Vec<&Dependency>> {
		let mut ret_vec: Vec<&Dependency> = Vec::new();

		if let Some(dep_list) = self.get_depends("Depends") {
			for dep in dep_list {
				ret_vec.push(dep)
			}
		}
		if let Some(dep_list) = self.get_depends("PreDepends") {
			for dep in dep_list {
				ret_vec.push(dep)
			}
		}
		if ret_vec.is_empty() {
			return None;
		}
		Some(ret_vec)
	}

	/// Returns a Reference Vector, if it exists, for "Recommends".
	pub fn recommends(&self) -> Option<&Vec<Dependency>> { self.get_depends("Recommends") }

	/// Returns a Reference Vector, if it exists, for "suggests".
	pub fn suggests(&self) -> Option<&Vec<Dependency>> { self.get_depends("Suggests") }

	/// Get the translated long description
	pub fn description(&self) -> String {
		let mut records = self.records.borrow_mut();
		records.lookup_desc(&self.ptr.desc);
		records.description()
	}

	/// Get the translated short description
	pub fn summary(&self) -> String {
		let mut records = self.records.borrow_mut();
		records.lookup_desc(&self.ptr.desc);
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

		if let Some(pkg_file) = package_files.iter().next() {
			let mut records = self.records.borrow_mut();
			records.lookup_ver(pkg_file);
			return records.hash_find(hash_type);
		}
		None
	}

	/// Returns an iterator of URIs for the version
	pub fn uris(&'a self) -> impl Iterator<Item = String> + 'a {
		self.file_list
			.get_or_init(|| self.gen_file_list())
			.iter()
			.filter_map(|package_file| {
				let mut records = self.records.borrow_mut();
				records.lookup_ver(package_file);

				let uri = records.uri(package_file);
				if !uri.starts_with("file:") {
					Some(uri)
				} else {
					None
				}
			})
	}

	/// Internal Method for converting apt::deps into rust-apt deps
	fn convert_depends(&self, apt_deps: apt::DepContainer) -> Dependency {
		let mut base_vec = Vec::new();
		for base_dep in apt_deps.dep_list {
			base_vec.push(BaseDep {
				records: Rc::clone(&self.records),
				apt_dep: base_dep,
			})
		}
		Dependency {
			dep_type: apt_deps.dep_type,
			base_deps: base_vec,
		}
	}

	/// Internal Method for Generating the Dependency HashMap
	fn gen_depends(&self) -> HashMap<String, Vec<Dependency>> {
		let mut dependencies: HashMap<String, Vec<Dependency>> = HashMap::new();
		for dep in apt::dep_list(&self.ptr) {
			if let Some(vec) = dependencies.get_mut(&dep.dep_type) {
				vec.push(self.convert_depends(dep))
			} else {
				dependencies.insert(dep.dep_type.to_owned(), vec![self.convert_depends(dep)]);
			}
		}
		dependencies
	}
}

impl<'a> fmt::Display for Version<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{}: Version {} <ID: {}, arch: {}, size: {}, installed_size: {}, section: {} Priority \
			 {} at {}, downloadable: {}>",
			self.pkgname(),
			self.version(),
			self.id(),
			self.arch(),
			unit_str(self.size(), NumSys::Decimal),
			unit_str(self.installed_size(), NumSys::Decimal),
			self.section(),
			self.priority_str(),
			self.priority(),
			self.downloadable(),
		)?;
		Ok(())
	}
}

/// A struct representing a Base Dependency
#[derive(Debug)]
pub struct BaseDep {
	apt_dep: apt::BaseDep,
	records: Rc<RefCell<Records>>,
}

impl BaseDep {
	pub fn name(&self) -> &String { &self.apt_dep.name }

	pub fn version(&self) -> &String { &self.apt_dep.version }

	pub fn comp(&self) -> &String {
		&self.apt_dep.comp
		// self.apt_dep.Comp_Type()
	}

	pub fn dep_type(&self) -> &String { &self.apt_dep.dep_type }

	pub fn all_targets(&self) -> impl Iterator<Item = Version> {
		apt::dep_all_targets(&self.apt_dep)
			.into_iter()
			.map(|ver_ptr| Version::new(Rc::clone(&self.records), ver_ptr))
	}
}

impl fmt::Display for BaseDep {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"BaseDep <Name: {}, Version: {}, Comp: {}, Type: {}>",
			self.name(),
			self.version(),
			self.comp(),
			self.dep_type(),
		)?;
		Ok(())
	}
}

/// A struct representing an Or_Group of Dependencies
#[derive(Debug)]
pub struct Dependency {
	pub dep_type: String,
	pub base_deps: Vec<BaseDep>,
}

impl Dependency {
	/// Returns True if there are multiple dependencies that can satisfy this
	pub fn is_or(&self) -> bool { self.base_deps.len() > 1 }

	/// Returns a reference to the first BaseDep
	pub fn first(&self) -> &BaseDep { &self.base_deps[0] }
}

impl fmt::Display for Dependency {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.is_or() {
			write!(f, "Or Dependencies[")?;
		} else {
			write!(f, "Dependency[")?;
		}
		for dep in &self.base_deps {
			write!(
				f,
				"\n    BaseDep <Name: {}, Version: {}, Comp: {}, Type: {}>,",
				dep.name(),
				dep.version(),
				dep.comp(),
				dep.dep_type(),
			)?;
		}
		write!(f, "\n]")?;
		Ok(())
	}
}
