#pragma once
#include "rust/cxx.h"
#include <apt-pkg/cachefile.h>
#include <apt-pkg/debfile.h>
#include <apt-pkg/error.h>
#include <apt-pkg/fileutl.h>
#include <apt-pkg/indexfile.h>
#include <apt-pkg/pkgcache.h>
#include <apt-pkg/policy.h>
#include <apt-pkg/sourcelist.h>
#include <apt-pkg/update.h>
#include <sstream>
#include <string>

#include "oma-apt/src/raw/cache.rs"
#include "oma-apt/src/raw/progress.rs"

/// Update the package lists, handle errors and return a Result.
inline void Cache::update(DynAcquireProgress& callback) const {
	AcqTextStatus progress(callback);

	ListUpdate(progress, *ptr->GetSourceList(), pulse_interval(callback));
	handle_errors();
}

// Return a package by name.
inline Package Cache::unsafe_find_pkg(rust::string name) const noexcept {
	return Package{ std::make_unique<PkgIterator>(
	safe_get_pkg_cache(ptr.get())->FindPkg(name.c_str())) };
}

inline Package Cache::begin() const {
	return Package{ std::make_unique<PkgIterator>(safe_get_pkg_cache(ptr.get())->PkgBegin()) };
}

/// The priority of the package as shown in `apt policy`.
inline int32_t Cache::priority(const Version& ver) const noexcept {
	return ptr->GetPolicy()->GetPriority(*ver.ptr);
}

inline DepCache Cache::create_depcache() const noexcept {
	return DepCache{ std::make_unique<PkgDepCache>(ptr->GetDepCache()) };
}

inline std::unique_ptr<Records> Cache::create_records() const noexcept {
	return Records::Unique(ptr);
}

inline void Cache::find_index(PackageFile& pkg_file) const noexcept {
	if (!pkg_file.index_file) {
		pkgIndexFile* index;

		if (!ptr->GetSourceList()->FindIndex(*pkg_file.ptr, index)) {
			_system->FindIndex(*pkg_file.ptr, index);
		}
		pkg_file.index_file = std::make_unique<IndexFile>(index);
	}
}

inline rust::Vec<rust::string> Cache::show_broken_package(rust::Vec<rust::string> &result, const Package& pkg, bool now) const noexcept {
	PkgIterator const& Pkg = *pkg.ptr;
	ptr->GetDepCache();
	PkgCacheFile* const Cache = &*ptr;
	bool Now = now;

	if (Now == true) {
		if ((*Cache)[Pkg].NowBroken() == false) return result;
	} else {
		if ((*Cache)[Pkg].InstBroken() == false) return result;
	}

    std::stringstream s;

	// Print out each package and the failed dependencies
	s << Pkg.FullName(true) << " :";
	unsigned const Indent = Pkg.FullName(true).size() + 3;
	bool First = true;
	pkgCache::VerIterator Ver;

	if (now == true)
		Ver = Pkg.CurrentVer();
	else
		Ver = (*Cache)[Pkg].InstVerIter(*Cache);

	if (Ver.end() == true) {
		s << std::endl;
		result.push_back(s.str());
		return result;
	}

	for (pkgCache::DepIterator D = Ver.DependsList(); D.end() == false;) {
		// Compute a single dependency element (glob or)
		pkgCache::DepIterator Start;
		pkgCache::DepIterator End;
		D.GlobOr(Start, End); // advances D

		if ((*Cache)->IsImportantDep(End) == false) continue;

		if (Now == true) {
			if (((*Cache)[End] & pkgDepCache::DepGNow) == pkgDepCache::DepGNow)
				continue;
		} else {
			if (((*Cache)[End] & pkgDepCache::DepGInstall) == pkgDepCache::DepGInstall)
				continue;
		}

		bool FirstOr = true;
		while (1) {
			if (First == false)
				for (unsigned J = 0; J != Indent; J++)
					s << ' ';
			First = false;

			if (FirstOr == false) {
				for (unsigned J = 0; J != strlen(End.DepType()) + 3; J++)
					s << ' ';
			} else
				s << ' ' << End.DepType() << ": ";
			FirstOr = false;

			s << Start.TargetPkg().FullName(true);

			// Show a quick summary of the version requirements
			if (Start.TargetVer() != 0)
				s << " (" << Start.CompType() << " " << Start.TargetVer() << ")";

			/* Show a summary of the target package if possible. In the case
			   of virtual packages we show nothing */
			pkgCache::PkgIterator Targ = Start.TargetPkg();
			if (Targ->ProvidesList == 0) {
				s << ' ';
				pkgCache::VerIterator Ver = (*Cache)[Targ].InstVerIter(*Cache);
				if (now == true) Ver = Targ.CurrentVer();

				if (Ver.end() == false) {
					if (Now == true)
						ioprintf(std::cout, "but %s is installed", Ver.VerStr());
					else
						ioprintf(std::cout, "but %s is to be installed", Ver.VerStr());
				} else {
					if ((*Cache)[Targ].CandidateVerIter(*Cache).end() == true) {
						if (Targ->ProvidesList == 0)
							s << "but it is to be installed";
						else
							s << "but it is a virtual package";
					} else
						s << (Now ? "but it is not installed" : "but it is not going to be installed");
				}
			}

			if (Start != End) std::cout << " or";
			s << std::endl;
			result.push_back(s.str());

			if (Start == End) break;
			++Start;
		}
	}

	return result;
}

inline rust::Vec<rust::string> Cache::show_broken(bool const Now) const noexcept {
	// convert PkgCacheFile to pkgDepCache
	// apt-pkg/cachefile.h: operator pkgDepCache &() const {return *DCache;};
	rust::vec<rust::string> result;
	if (((pkgDepCache&) ptr).BrokenCount() == 0) return result;
	
	//    out << "The following packages have unmet dependencies:" << std::endl;
	APT::PackageUniverse Universe(&*ptr);
	for (auto const& Pkg : Universe)
		show_broken_package(result, Package{ std::make_unique<PkgIterator>(Pkg) }, Now);

	return result;
}

/// These should probably go under a index file binding;
/// Return true if the PackageFile is trusted.
inline bool Cache::is_trusted(PackageFile& pkg_file) const noexcept {
	this->find_index(pkg_file);
	return (*pkg_file.index_file)->IsTrusted();
}

/// Get the package list uris. This is the files that are updated with `apt update`.
inline rust::Vec<SourceURI> Cache::source_uris() const noexcept {
	pkgAcquire fetcher;
	rust::Vec<SourceURI> list;

	ptr->GetSourceList()->GetIndexes(&fetcher, true);
	pkgAcquire::UriIterator I = fetcher.UriBegin();
	for (; I != fetcher.UriEnd(); ++I) {
		list.push_back(SourceURI{ I->URI, flNotDir(I->Owner->DestFile) });
	}
	return list;
}

inline Cache create_cache(rust::Slice<const rust::String> deb_files) {
	std::unique_ptr<pkgCacheFile> cache = std::make_unique<pkgCacheFile>();

	for (auto deb_str : deb_files) {
		std::string deb_string(deb_str.c_str());

		// Make sure this is a valid archive.
		// signal: 11, SIGSEGV: invalid memory reference
		FileFd fd(deb_string, FileFd::ReadOnly);
		debDebFile debfile(fd);
		handle_errors();

		// Add the deb to the cache.
		if (!cache->GetSourceList()->AddVolatileFile(deb_string)) {
			_error->Error(
			"%s", ("Couldn't add '" + deb_string + "' to the cache.").c_str());
			handle_errors();
		}

		handle_errors();
	}

	return Cache{ std::move(cache) };
}
