#pragma once
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

#include "rust/cxx.h"

// This has to come first for now
// It defines the SourceURI
#include "oma-apt/src/raw/package.rs"

#include "depcache.h"
#include "records.h"
#include "types.h"

// Defines the callbacks code that's generated for progress
#include "oma-apt/src/raw/progress.rs"

class PkgCacheFile : public pkgCacheFile {
   public:
	// Maybe we use this if we don't want pin_mut() all over the place in Rust.
	PkgCacheFile* unconst() const { return const_cast<PkgCacheFile*>(this); }

	/// Update the package lists, handle errors and return a Result.
	void u_update(DynAcquireProgress& callback) const {
		AcqTextStatus progress(callback);

		ListUpdate(progress, *this->unconst()->GetSourceList(), pulse_interval(callback));
		handle_errors();
	}

	// Return a package by name.
	std::unique_ptr<PkgIterator> u_find_pkg(rust::string name) const {
		return std::make_unique<PkgIterator>(this->unconst()->GetPkgCache()->FindPkg(name.c_str()));
	}

	std::unique_ptr<PkgIterator> u_begin() const {
		return std::make_unique<PkgIterator>(this->unconst()->GetPkgCache()->PkgBegin());
	}

	/// The priority of the package as shown in `apt policy`.
	int32_t priority(const VerIterator& ver) const {
		return this->unconst()->GetPolicy()->GetPriority(ver);
	}

	std::unique_ptr<PkgDepCache> create_depcache() const {
		return std::make_unique<PkgDepCache>(this->unconst()->GetDepCache());
	}

	std::unique_ptr<PkgRecords> create_records() const {
		return PkgRecords::Unique(this->unconst());
	}

	std::unique_ptr<IndexFile> find_index(const PkgFileIterator& file) const {
		pkgIndexFile* index;
		if (!this->unconst()->GetSourceList()->FindIndex(file, index)) {
			_system->FindIndex(file, index);
		}
		return std::make_unique<IndexFile>(index);
	}

	bool is_trusted(const IndexFile& file) const { return file->IsTrusted(); }

	/// Get the package list uris. This is the files that are updated with `apt update`.
	rust::Vec<SourceURI> source_uris() const {
		pkgAcquire fetcher;
		rust::Vec<SourceURI> list;

		this->unconst()->GetSourceList()->GetIndexes(&fetcher, true);
		pkgAcquire::UriIterator I = fetcher.UriBegin();
		for (; I != fetcher.UriEnd(); ++I) {
			list.push_back(SourceURI{I->URI, flNotDir(I->Owner->DestFile)});
		}
		return list;
	}

	PkgCacheFile() : pkgCacheFile(){};
};

inline std::unique_ptr<PkgCacheFile> u_create_cache(rust::Slice<const rust::String> volatile_files
) {
	std::unique_ptr<PkgCacheFile> cache = std::make_unique<PkgCacheFile>();

	for (auto file_str : volatile_files) {
		std::string file_string(file_str.c_str());

		// Add the file to the cache.
		if (!cache->GetSourceList()->AddVolatileFile(file_string)) {
			_error->Error("%s", ("Couldn't add '" + file_string + "' to the cache.").c_str());
		}
	}

	// Building the pkg caches can cause an error that might not
	// Get propagated until you get a pkg which shouldn't have errors.
	// See https://gitlab.com/volian/rust-apt/-/issues/24
	cache->GetPkgCache();
	handle_errors();

	return cache;
}
