# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.1 (2023-08-18)

### Chore

 - <csr-id-b617647574eca3f4856374eb0b4aeea300a00921/> Set version as 0.1.0
 - <csr-id-ed56e40168ca8b826c2ed81bc164db4bddf056c3/> Set name as oma-apt
 - <csr-id-ccf3dcba047042edbe5fa0d6f5de02f21d6950e5/> Add vscode typing helper for c++ files
 - <csr-id-a0277868287910924a03ef80b611d0a23f6caf4f/> Clippy: unused import
 - <csr-id-a60323705ff67d59dc565be87cb12ac765828a88/> Cripple some examples on ye-old-apt
 - <csr-id-a5b8ede7ece2d783c8c9d0b8d56cfc3a2ff3883b/> Format
 - <csr-id-52213cdd72b708323a2611ec926d372bd6b6d6df/> Dev itertools -> 0.9
 - <csr-id-6d16f0987fd3b1bdda47e2fda487518e8e065d6c/> Bump travis

### Documentation

 - <csr-id-e63f8076bb5b67afba647ec0dce63839066dd9f5/> Update `README.md`
 - <csr-id-deb30c69d4bbb28a501cb484eb35d1be190dad99/> Deploy rust documentation on all commits and MRs
 - <csr-id-a4cf64d32d2031dee7260506d865e889e6831a0d/> Fix error in example
 - <csr-id-9c137f06c8465e1d17e54ccc3b370e05a0828a2c/> Fix wrong comment in tests
 - <csr-id-037ada84b5702738c9625d24999d570cca9eed1b/> Update readme
 - <csr-id-ef3fbc2f14923c5090369e0451630bab03a76646/> Add documentation to methods

### New Features

 - <csr-id-916e6f4f6efcf25c1b3634575579aca9176074e5/> Implement bindings for action groups
 - <csr-id-6de58a3c3a4b553f40222d0015dfdc8ac1ea004b/> Add 'rdepends_map' method to package struct
 - <csr-id-51ac3eae5fd5997ecdd807a851b151612470dfe5/> Add `fix_broken` method on the cache.
 - <csr-id-119ded0d276f7951f35d08bc2ddf2cb4ebda5807/> Allow globbing of packages
 - <csr-id-430c5aa8f040b2e73fa5bee6f08e037228608c9d/> Add support for installing local '.deb' files
 - <csr-id-81216c920edce01a9d22f496fe13d661c8e67561/> Ability to parse files in DEB822 format
 - <csr-id-cda4df82ec77eb599359fa7eebd5723d1807c053/> Add info accessor for pkg records
 - <csr-id-1ddec8ca3c7c74b91d918d23a21fd8dcd9c3a3f2/> Add cache modification functionality
 - <csr-id-64e2ccbe1e81729e8113c26cb85a4c0b4908bae5/> Add version origin accessors
 - <csr-id-8db639ad63831bd1bdc2c63ccf9062a92840dad8/> Get parent package from version
 - <csr-id-60f93415af5bb8b70805216d3c9b13960f31a95e/> Add bindings for version comparisons
 - <csr-id-15d83bb77c0829fa2e452e05d4097a603de8fb01/> Add method for package name without arch
 - <csr-id-089b0905d3ccaf9ef91f5b36aa058b23e31f9aa5/> Add bindings to apt's configuration
 - <csr-id-964e0b617b2ab481372f53ad810b2bf8aa0471ca/> Add support for updating the package list
 - <csr-id-88c38a545175da83a315138d0811b876e87ddd5c/> Add new sorting methods
 - <csr-id-f3f332c7326ded1832d64c38e4e8eaf97e98eef8/> Add new methods for cache information
 - <csr-id-1207611a1c505c882a6c25e0b439ca21bdcd6526/> Allow sorting the packages by name with PackageSort
 - <csr-id-ce805105c22e1ace6a626759aca6b59aff36aea8/> Add initial support for getting dependencies
 - <csr-id-e6e889f63a01e974d6320e93c8bf5cb68cc7db1a/> Add provides method to the cache
 - <csr-id-2b2a8d04db1c5e1f4963e92f8ec07fdc296bf1bc/> Add more depcache state checks
 - <csr-id-8e223cdfd9ed6965f754f0e6e79a704bd4f4a7f8/> Add ability to get source uris
 - <csr-id-c47f4a2a166b2bd82323397eab72b3698daf688a/> Add `is_installed` method to package
 - <csr-id-68f261c7fa99f623967499273184a39bfc494c57/> Add ability to get package hashes
 - <csr-id-334dfcb1de31dd3f93af768ab43319db4b2f24b9/> Allow `PackageSort` to be 'built'
 - <csr-id-d616a362bed637b6eadb25f92bc92f845fff30a0/> Add more fields to the package object
 - <csr-id-cd310e06d067bdc587c255c421492dc91fad317f/> Add more version fields
   pkgname, size, installed_size, downloadable, id
 - <csr-id-fcd4f97e4fa45a1ab6acf12860c840f844c1baca/> Add `summary` and `description` for versions
 - <csr-id-7f30d9ba0127bf44cde80f5456608f25dd23f361/> Add `is_installed` on the version

### Bug Fixes

 - <csr-id-daafbb945f7402c855fd012880a396d2eef52843/> Not copying packages into archive
 - <csr-id-481f4e28bf8b3b3132a7c5c23f35466a087e281e/> Segfault when a package list is corrupt
 - <csr-id-15e6df8711975a6905bcb46f9b7ed873446a1633/> Redepends not returning the parent's version.
 - <csr-id-c6d099ca50da3d3d8f28c6e80b01cd37005c6f94/> Missing import for terminal size
 - <csr-id-9ec3d8e84e3fbd20616a8d85387ade28e95ad934/> Replace termsize with terminal_size
 - <csr-id-1aa839d108427bc903f6ce1b0ef7d4c663b0d235/> Panic when generating depends for a package that has none
 - <csr-id-8dd5061314ff7fa699a3b0c3d0658a8d66004603/> Remove test print
 - <csr-id-7f9c9589b6bf454b47c41e14656665d86438b579/> Performance regression from `DepCache` init
 - <csr-id-3b7f86dd01c14b86470b23d9f91a90f71d795053/> Invalid memory reference inserting non deb files into the cache
 - <csr-id-98e7092451c3c591fa0d89718f05150ceb03b74c/> Typo in comment
 - <csr-id-6eb01851a69311a647647ff6702b5c4cd1f55d9e/> Pkg list returns only virtual packages instead of including them with real pkgs
 - <csr-id-57b24dbd699bc41b309cfc26032450f8a9a0676d/> Wrong integer types from some functions
 - <csr-id-c89049197b6ce27c7e8b67d592c1a0e4ad9a92bf/> Several memory leaks
 - <csr-id-dfacd8edf7ef96072142b33293f8f7265516964b/> Give version structs their own pointer.
 - <csr-id-99fba9024ab7c0797ddb468e72c42dd612f28934/> Missing header(?)
   This seems to be required on 20.04.
 - <csr-id-ecd60906d21b8bb65dd845b3e663053ae8098a49/> Extra headers required on newer apt

### Other

 - <csr-id-496511ad09ca8f997df28dd1d38971977ed2680d/> Derive debug on Config struct
 - <csr-id-a26f811dcc2a24238ef2880f1c255649a2488e65/> Speed up with cache
 - <csr-id-4fccf1b60cbcf8aaac97736caa73fb528906b8f1/> Release version `0.4.1`
   This release also removes its ties with apt-pkg-native-rs as none of the code is from the original project
 - <csr-id-594ff856c71050f57518b1b5e19d732358243783/> Remove `dev.py` script in favor of justfile
 - <csr-id-df102936ae6616211ee7ef4a5b16b5a1bf5e30c6/> Add ci configuration
 - <csr-id-bc911023c3c54feec3e1e12dc9e391696a7d31d4/> Bump version for release
 - <csr-id-9dcabf0d9a8869983928a81bfb35361bd86b2cc2/> Add `dev.py` to improve workflow
 - <csr-id-8a6e927f78725d5e30e075843ad46eb0352e3a96/> Greatly improve crate tests
 - <csr-id-75f94717388e9bebacccba7b1b5c238aecfdaa32/> Bump version for release
 - <csr-id-0e55dfaf914ef40fb2a6d542e52490e154a52a50/> Convert VerIterators into VersionPtr
 - <csr-id-6dbe4f7a5edfd6b03749620848429b2bdd8b197c/> Add simple test
 - <csr-id-3d66d108b2aff2c273d9beb41daa8bc2031c1b3a/> Work around https://github.com/rust-lang/rust/pull/57018
 - <csr-id-f8c52cba964b172b0878a7eeb7b022f24f6e8bea/> Bump to xenial
 - <csr-id-46aa9ed07f76ebe4c50de14c4b35914b07b9edd7/> Support building with very old apt
 - <csr-id-d13db3450bf76037e5e5a3bad0b4a9c20b57deb1/> Version exposed
 - <csr-id-3314733ebb4990d447cfb1132150fbfcb7db78c6/> Fixed iterators, "origin"-like concept exposed

### Performance

 - <csr-id-f14d3508e1f86e57ef400ccb4b05c672711e1926/> Add bool to skip the `pkgDepCache` for `is_upgradable``
 - <csr-id-9cd6d06fb5eb4760930d98d5ab7ae63bfa6fc05e/> Sort packages before crossing the barrier
 - <csr-id-b16ee9a4bb00a920953f49428be522df988b15e9/> Separate depcache into new struct

### Refactor

 - <csr-id-9a4e2595911d05914bf28b4113ed8fe594b76485/> Split `raw` bindings into submodules
 - <csr-id-f40f4d25ba7cea9a495188aead8dc79436f0338b/> Move utils into `util.rs`
 - <csr-id-68694c1aa4f0bfbca7907be54147402eb87340b1/> Send owned string across cxx barrier
 - <csr-id-f74ffe7df31df1775eeaec93477259d95f79e9b2/> Remove unnecessary clippy config
 - <csr-id-5ca96dd0258b48fb5314fcc18db8375b02483345/> Improve unit_str
 - <csr-id-bd8977b58df42b444f947cabbc8d430174a3453c/> Add TB to unit_str
 - <csr-id-268b7e6dd4f9b9227b14396a88594fda80675730/> Clean up c++ code
 - <csr-id-20c933000c9229c91aa4a7a9e5c0d5862d0e402f/> Change PackageSort methods to set true
 - <csr-id-94de3cb53a09dc51155d733a0da26acaaf817875/> Format c++
 - <csr-id-d30735b71d7c69be4606ff1e3f6007984b039d95/> Organize functions and update docs
 - <csr-id-d4779b883332ae406103eba7e1cd6c7567d0b472/> Change fields to methods
 - <csr-id-641d8be8d3ab5e8f861ba96bfb3f403214d80559/> Remove compile commands as it's env dependent
 - <csr-id-d58ed083f365369548293f7046c23a9353d52725/> Clean and format code
 - <csr-id-963b08074284f8005257bb136587f94b0f17f548/> Convert cache to unique_ptr
 - <csr-id-9fd24dc76122c25033cdbc930b3bac2782d83f52/> Change depends to use shared_ptr
 - <csr-id-7101eb0afd293321e61b314eb0eaf0e2cf1957ef/> Convert Records and PackageFile into UniquePtr
 - <csr-id-a2de4eace1a7a58aec9fd71375e8af4154bb0745/> Convert PkgIterator into PackagePtr
 - <csr-id-6ad07fd488a32941108226a012f565be8cefef49/> Remove ver_release
 - <csr-id-080c5661140a6208766cab44d2bd607a786ed397/> Use unique_ptr for version pointer
 - <csr-id-66b553b323c0289051b9c9b6fc61ca1a6e0c26e2/> Change `find_name` to use String
 - <csr-id-92d947c3986ce211a84722a27ea54b7934098f46/> Split Package and Version object into separate file
 - <csr-id-8df9d5fe8b4da3dc0fbef98c1d3d0d5019195d48/> Change `char`s to `rust::string`
 - <csr-id-d525b4fca66fb3719c50a9c68315848924113edb/> Simplify the code for the `get` method
 - <csr-id-7dd85606e66f9cab6418d3d0018bb84333f7fccd/> Use vector for `sorted` instead of `BTreeMap`
 - <csr-id-81f5174203f1398950fe1d302d0e165243ac9903/> Build pkg pointer iterator on the fly
 - <csr-id-68ad295230fde116a83d981bf6bf596eebf7f49d/> Convert package getters into iterators
 - <csr-id-8cfc62e14e33a17ce8dabf75134e732f18e06c24/> Make uri method an iterator
 - <csr-id-66a2eb141164a401d89d3855d9d4309fb8f2aae3/> Change `get_uris` to just `uris`
 - <csr-id-6b8a9a43a876c9497a521912b8df0aeb0794e67e/> Change variable names
 - <csr-id-c930a7226d04896effc0a20096ec10eaa1ec1737/> Store last lookup to prevent unnecessary lookups
 - <csr-id-59b1144921f77870874d7f84b66c0e3513afba67/> New `rustfmt.toml`
 - <csr-id-7103b4e6fe1057ef172b1dff25af354d8efd56f7/> Change indentation to the superior tab
 - <csr-id-8496109b71ea5653a71749f8779f6d002d2516fb/> Comment unused functions
 - <csr-id-5ca9a50e7e3248e57c43aee465e56c5bc92b11e8/> Satisfy clippy and organize code
 - <csr-id-663ae5e0d0bfe6c730d956b25b877c4588c55a5f/> Use `Rc<RefCell>` for the records struct.
 - <csr-id-1ed41ae5b876f08ad8416c07f491d42a1966af64/> Rename types and use C++ Opaque Types

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 194 commits contributed to the release over the course of 2227 calendar days.
 - 113 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#1](https://github.com/AOSC-Dev/oma-apt/issues/1)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#1](https://github.com/AOSC-Dev/oma-apt/issues/1)**
    - Add a reload() ([`b104724`](https://github.com/AOSC-Dev/oma-apt/commit/b104724ccfc4e6d7877972b4c868f3aa49df0593))
 * **Uncategorized**
    - Set version as 0.1.0 ([`b617647`](https://github.com/AOSC-Dev/oma-apt/commit/b617647574eca3f4856374eb0b4aeea300a00921))
    - Set name as oma-apt ([`ed56e40`](https://github.com/AOSC-Dev/oma-apt/commit/ed56e40168ca8b826c2ed81bc164db4bddf056c3))
    - Not copying packages into archive ([`daafbb9`](https://github.com/AOSC-Dev/oma-apt/commit/daafbb945f7402c855fd012880a396d2eef52843))
    - Implement bindings for action groups ([`916e6f4`](https://github.com/AOSC-Dev/oma-apt/commit/916e6f4f6efcf25c1b3634575579aca9176074e5))
    - Segfault when a package list is corrupt ([`481f4e2`](https://github.com/AOSC-Dev/oma-apt/commit/481f4e28bf8b3b3132a7c5c23f35466a087e281e))
    - Redepends not returning the parent's version. ([`15e6df8`](https://github.com/AOSC-Dev/oma-apt/commit/15e6df8711975a6905bcb46f9b7ed873446a1633))
    - Missing import for terminal size ([`c6d099c`](https://github.com/AOSC-Dev/oma-apt/commit/c6d099ca50da3d3d8f28c6e80b01cd37005c6f94))
    - Replace termsize with terminal_size ([`9ec3d8e`](https://github.com/AOSC-Dev/oma-apt/commit/9ec3d8e84e3fbd20616a8d85387ade28e95ad934))
    - Derive debug on Config struct ([`496511a`](https://github.com/AOSC-Dev/oma-apt/commit/496511ad09ca8f997df28dd1d38971977ed2680d))
    - Add 'rdepends_map' method to package struct ([`6de58a3`](https://github.com/AOSC-Dev/oma-apt/commit/6de58a3c3a4b553f40222d0015dfdc8ac1ea004b))
    - Add `fix_broken` method on the cache. ([`51ac3ea`](https://github.com/AOSC-Dev/oma-apt/commit/51ac3eae5fd5997ecdd807a851b151612470dfe5))
    - Update `README.md` ([`e63f807`](https://github.com/AOSC-Dev/oma-apt/commit/e63f8076bb5b67afba647ec0dce63839066dd9f5))
    - Panic when generating depends for a package that has none ([`1aa839d`](https://github.com/AOSC-Dev/oma-apt/commit/1aa839d108427bc903f6ce1b0ef7d4c663b0d235))
    - Remove dev debug print ([`358b89a`](https://github.com/AOSC-Dev/oma-apt/commit/358b89ae28b95e94d412dc561cab97e7dc92fc57))
    - Release `0.5.0` ([`12db48d`](https://github.com/AOSC-Dev/oma-apt/commit/12db48ddc102cc9ec9d42d0c1c0aae3d9eef7bfd))
    - Allow Packages and Versions to be put in a hashmap ([`6a8146e`](https://github.com/AOSC-Dev/oma-apt/commit/6a8146e47a755ff7e219d0bc28fc1fe2161b044b))
    - Add higher level struct for Provides ([`dc09d71`](https://github.com/AOSC-Dev/oma-apt/commit/dc09d718a4023886049e1194e6b3722ab9230f8b))
    - Fix candidate returning wrong version after set_candidate ([`5246a09`](https://github.com/AOSC-Dev/oma-apt/commit/5246a09f9dc7e6e74715d8fae23a06374856a194))
    - Change `full_name` => `fullname` ([`f05f916`](https://github.com/AOSC-Dev/oma-apt/commit/f05f916ca5c75a1412ed4596462f79a5f41e86d1))
    - Fix version list of virtual packages causing panic ([`b983950`](https://github.com/AOSC-Dev/oma-apt/commit/b98395000a0e8ee4b6573b29d0544dd7ffc0b25a))
    - Clippy changes ([`9632a55`](https://github.com/AOSC-Dev/oma-apt/commit/9632a555d3f15b20648ead2a7850cca3b971bafe))
    - Major Restructure ([`087ea68`](https://github.com/AOSC-Dev/oma-apt/commit/087ea68534f03499920a55acdf5e51d3c3f41fec))
    - Fix issue with resolving '.deb' packages ([`2f1633d`](https://github.com/AOSC-Dev/oma-apt/commit/2f1633d26c9dee69d5852d1fcbf84b2586876555))
    - Fix incorrect filename in   'build.rs' ([`d782cb0`](https://github.com/AOSC-Dev/oma-apt/commit/d782cb01814c747c3dde82b23a8d4eac200bbff8))
    - Remove test print ([`8dd5061`](https://github.com/AOSC-Dev/oma-apt/commit/8dd5061314ff7fa699a3b0c3d0658a8d66004603))
    - Performance regression from `DepCache` init ([`7f9c958`](https://github.com/AOSC-Dev/oma-apt/commit/7f9c9589b6bf454b47c41e14656665d86438b579))
    - Allow globbing of packages ([`119ded0`](https://github.com/AOSC-Dev/oma-apt/commit/119ded0d276f7951f35d08bc2ddf2cb4ebda5807))
    - Ensure a blank string isn't passed in to TagSection instances ([`7750594`](https://github.com/AOSC-Dev/oma-apt/commit/7750594be3ca30ee29804bf46cf76896dd61655b))
    - Invalid memory reference inserting non deb files into the cache ([`3b7f86d`](https://github.com/AOSC-Dev/oma-apt/commit/3b7f86dd01c14b86470b23d9f91a90f71d795053))
    - Speed up with cache ([`a26f811`](https://github.com/AOSC-Dev/oma-apt/commit/a26f811dcc2a24238ef2880f1c255649a2488e65))
    - Add support for installing local '.deb' files ([`430c5aa`](https://github.com/AOSC-Dev/oma-apt/commit/430c5aa8f040b2e73fa5bee6f08e037228608c9d))
    - Ability to parse files in DEB822 format ([`81216c9`](https://github.com/AOSC-Dev/oma-apt/commit/81216c920edce01a9d22f496fe13d661c8e67561))
    - Add info accessor for pkg records ([`cda4df8`](https://github.com/AOSC-Dev/oma-apt/commit/cda4df82ec77eb599359fa7eebd5723d1807c053))
    - Release version `0.4.1` ([`4fccf1b`](https://github.com/AOSC-Dev/oma-apt/commit/4fccf1b60cbcf8aaac97736caa73fb528906b8f1))
    - Add cache modification functionality ([`1ddec8c`](https://github.com/AOSC-Dev/oma-apt/commit/1ddec8ca3c7c74b91d918d23a21fd8dcd9c3a3f2))
    - Typo in comment ([`98e7092`](https://github.com/AOSC-Dev/oma-apt/commit/98e7092451c3c591fa0d89718f05150ceb03b74c))
    - Deploy rust documentation on all commits and MRs ([`deb30c6`](https://github.com/AOSC-Dev/oma-apt/commit/deb30c69d4bbb28a501cb484eb35d1be190dad99))
    - Add version origin accessors ([`64e2ccb`](https://github.com/AOSC-Dev/oma-apt/commit/64e2ccbe1e81729e8113c26cb85a4c0b4908bae5))
    - Split `raw` bindings into submodules ([`9a4e259`](https://github.com/AOSC-Dev/oma-apt/commit/9a4e2595911d05914bf28b4113ed8fe594b76485))
    - Remove `dev.py` script in favor of justfile ([`594ff85`](https://github.com/AOSC-Dev/oma-apt/commit/594ff856c71050f57518b1b5e19d732358243783))
    - Get parent package from version ([`8db639a`](https://github.com/AOSC-Dev/oma-apt/commit/8db639ad63831bd1bdc2c63ccf9062a92840dad8))
    - Move utils into `util.rs` ([`f40f4d2`](https://github.com/AOSC-Dev/oma-apt/commit/f40f4d25ba7cea9a495188aead8dc79436f0338b))
    - Send owned string across cxx barrier ([`68694c1`](https://github.com/AOSC-Dev/oma-apt/commit/68694c1aa4f0bfbca7907be54147402eb87340b1))
    - Add bindings for version comparisons ([`60f9341`](https://github.com/AOSC-Dev/oma-apt/commit/60f93415af5bb8b70805216d3c9b13960f31a95e))
    - Some tests were also updated during this, along with the dev script. ([`52791ee`](https://github.com/AOSC-Dev/oma-apt/commit/52791eeffe6a16413a3e8cb1bf5ba78da4c2e288))
    - Add ci configuration ([`df10293`](https://github.com/AOSC-Dev/oma-apt/commit/df102936ae6616211ee7ef4a5b16b5a1bf5e30c6))
    - Fix error in example ([`a4cf64d`](https://github.com/AOSC-Dev/oma-apt/commit/a4cf64d32d2031dee7260506d865e889e6831a0d))
    - Add method for package name without arch ([`15d83bb`](https://github.com/AOSC-Dev/oma-apt/commit/15d83bb77c0829fa2e452e05d4097a603de8fb01))
    - Bump version for release ([`bc91102`](https://github.com/AOSC-Dev/oma-apt/commit/bc911023c3c54feec3e1e12dc9e391696a7d31d4))
    - Add bindings to apt's configuration ([`089b090`](https://github.com/AOSC-Dev/oma-apt/commit/089b0905d3ccaf9ef91f5b36aa058b23e31f9aa5))
    - Add support for updating the package list ([`964e0b6`](https://github.com/AOSC-Dev/oma-apt/commit/964e0b617b2ab481372f53ad810b2bf8aa0471ca))
    - Add new sorting methods ([`88c38a5`](https://github.com/AOSC-Dev/oma-apt/commit/88c38a545175da83a315138d0811b876e87ddd5c))
    - Fix wrong comment in tests ([`9c137f0`](https://github.com/AOSC-Dev/oma-apt/commit/9c137f06c8465e1d17e54ccc3b370e05a0828a2c))
    - Remove unnecessary clippy config ([`f74ffe7`](https://github.com/AOSC-Dev/oma-apt/commit/f74ffe7df31df1775eeaec93477259d95f79e9b2))
    - Add bool to skip the `pkgDepCache` for `is_upgradable`` ([`f14d350`](https://github.com/AOSC-Dev/oma-apt/commit/f14d3508e1f86e57ef400ccb4b05c672711e1926))
    - Add `dev.py` to improve workflow ([`9dcabf0`](https://github.com/AOSC-Dev/oma-apt/commit/9dcabf0d9a8869983928a81bfb35361bd86b2cc2))
    - Greatly improve crate tests ([`8a6e927`](https://github.com/AOSC-Dev/oma-apt/commit/8a6e927f78725d5e30e075843ad46eb0352e3a96))
    - Pkg list returns only virtual packages instead of including them with real pkgs ([`6eb0185`](https://github.com/AOSC-Dev/oma-apt/commit/6eb01851a69311a647647ff6702b5c4cd1f55d9e))
    - Add new methods for cache information ([`f3f332c`](https://github.com/AOSC-Dev/oma-apt/commit/f3f332c7326ded1832d64c38e4e8eaf97e98eef8))
    - Improve unit_str ([`5ca96dd`](https://github.com/AOSC-Dev/oma-apt/commit/5ca96dd0258b48fb5314fcc18db8375b02483345))
    - Wrong integer types from some functions ([`57b24db`](https://github.com/AOSC-Dev/oma-apt/commit/57b24dbd699bc41b309cfc26032450f8a9a0676d))
    - Add TB to unit_str ([`bd8977b`](https://github.com/AOSC-Dev/oma-apt/commit/bd8977b58df42b444f947cabbc8d430174a3453c))
    - Bump version for release ([`75f9471`](https://github.com/AOSC-Dev/oma-apt/commit/75f94717388e9bebacccba7b1b5c238aecfdaa32))
    - Update readme ([`037ada8`](https://github.com/AOSC-Dev/oma-apt/commit/037ada84b5702738c9625d24999d570cca9eed1b))
    - Clean up c++ code ([`268b7e6`](https://github.com/AOSC-Dev/oma-apt/commit/268b7e6dd4f9b9227b14396a88594fda80675730))
    - Change PackageSort methods to set true ([`20c9330`](https://github.com/AOSC-Dev/oma-apt/commit/20c933000c9229c91aa4a7a9e5c0d5862d0e402f))
    - Allow sorting the packages by name with PackageSort ([`1207611`](https://github.com/AOSC-Dev/oma-apt/commit/1207611a1c505c882a6c25e0b439ca21bdcd6526))
    - Sort packages before crossing the barrier ([`9cd6d06`](https://github.com/AOSC-Dev/oma-apt/commit/9cd6d06fb5eb4760930d98d5ab7ae63bfa6fc05e))
    - Format c++ ([`94de3cb`](https://github.com/AOSC-Dev/oma-apt/commit/94de3cb53a09dc51155d733a0da26acaaf817875))
    - Organize functions and update docs ([`d30735b`](https://github.com/AOSC-Dev/oma-apt/commit/d30735b71d7c69be4606ff1e3f6007984b039d95))
    - Change fields to methods ([`d4779b8`](https://github.com/AOSC-Dev/oma-apt/commit/d4779b883332ae406103eba7e1cd6c7567d0b472))
    - Remove compile commands as it's env dependent ([`641d8be`](https://github.com/AOSC-Dev/oma-apt/commit/641d8be8d3ab5e8f861ba96bfb3f403214d80559))
    - Clean and format code ([`d58ed08`](https://github.com/AOSC-Dev/oma-apt/commit/d58ed083f365369548293f7046c23a9353d52725))
    - Convert cache to unique_ptr ([`963b080`](https://github.com/AOSC-Dev/oma-apt/commit/963b08074284f8005257bb136587f94b0f17f548))
    - Change depends to use shared_ptr ([`9fd24dc`](https://github.com/AOSC-Dev/oma-apt/commit/9fd24dc76122c25033cdbc930b3bac2782d83f52))
    - Convert Records and PackageFile into UniquePtr ([`7101eb0`](https://github.com/AOSC-Dev/oma-apt/commit/7101eb0afd293321e61b314eb0eaf0e2cf1957ef))
    - Convert PkgIterator into PackagePtr ([`a2de4ea`](https://github.com/AOSC-Dev/oma-apt/commit/a2de4eace1a7a58aec9fd71375e8af4154bb0745))
    - Remove ver_release ([`6ad07fd`](https://github.com/AOSC-Dev/oma-apt/commit/6ad07fd488a32941108226a012f565be8cefef49))
    - Use unique_ptr for version pointer ([`080c566`](https://github.com/AOSC-Dev/oma-apt/commit/080c5661140a6208766cab44d2bd607a786ed397))
    - Convert VerIterators into VersionPtr ([`0e55dfa`](https://github.com/AOSC-Dev/oma-apt/commit/0e55dfaf914ef40fb2a6d542e52490e154a52a50))
    - Add initial support for getting dependencies ([`ce80510`](https://github.com/AOSC-Dev/oma-apt/commit/ce805105c22e1ace6a626759aca6b59aff36aea8))
    - Add vscode typing helper for c++ files ([`ccf3dcb`](https://github.com/AOSC-Dev/oma-apt/commit/ccf3dcba047042edbe5fa0d6f5de02f21d6950e5))
    - Add provides method to the cache ([`e6e889f`](https://github.com/AOSC-Dev/oma-apt/commit/e6e889f63a01e974d6320e93c8bf5cb68cc7db1a))
    - Change `find_name` to use String ([`66b553b`](https://github.com/AOSC-Dev/oma-apt/commit/66b553b323c0289051b9c9b6fc61ca1a6e0c26e2))
    - Split Package and Version object into separate file ([`92d947c`](https://github.com/AOSC-Dev/oma-apt/commit/92d947c3986ce211a84722a27ea54b7934098f46))
    - Add more depcache state checks ([`2b2a8d0`](https://github.com/AOSC-Dev/oma-apt/commit/2b2a8d04db1c5e1f4963e92f8ec07fdc296bf1bc))
    - Change `char`s to `rust::string` ([`8df9d5f`](https://github.com/AOSC-Dev/oma-apt/commit/8df9d5fe8b4da3dc0fbef98c1d3d0d5019195d48))
    - Separate depcache into new struct ([`b16ee9a`](https://github.com/AOSC-Dev/oma-apt/commit/b16ee9a4bb00a920953f49428be522df988b15e9))
    - Add ability to get source uris ([`8e223cd`](https://github.com/AOSC-Dev/oma-apt/commit/8e223cdfd9ed6965f754f0e6e79a704bd4f4a7f8))
    - Add `is_installed` method to package ([`c47f4a2`](https://github.com/AOSC-Dev/oma-apt/commit/c47f4a2a166b2bd82323397eab72b3698daf688a))
    - Simplify the code for the `get` method ([`d525b4f`](https://github.com/AOSC-Dev/oma-apt/commit/d525b4fca66fb3719c50a9c68315848924113edb))
    - Use vector for `sorted` instead of `BTreeMap` ([`7dd8560`](https://github.com/AOSC-Dev/oma-apt/commit/7dd85606e66f9cab6418d3d0018bb84333f7fccd))
    - Build pkg pointer iterator on the fly ([`81f5174`](https://github.com/AOSC-Dev/oma-apt/commit/81f5174203f1398950fe1d302d0e165243ac9903))
    - Convert package getters into iterators ([`68ad295`](https://github.com/AOSC-Dev/oma-apt/commit/68ad295230fde116a83d981bf6bf596eebf7f49d))
    - Add documentation to methods ([`ef3fbc2`](https://github.com/AOSC-Dev/oma-apt/commit/ef3fbc2f14923c5090369e0451630bab03a76646))
    - Make uri method an iterator ([`8cfc62e`](https://github.com/AOSC-Dev/oma-apt/commit/8cfc62e14e33a17ce8dabf75134e732f18e06c24))
    - Add ability to get package hashes ([`68f261c`](https://github.com/AOSC-Dev/oma-apt/commit/68f261c7fa99f623967499273184a39bfc494c57))
    - Change `get_uris` to just `uris` ([`66a2eb1`](https://github.com/AOSC-Dev/oma-apt/commit/66a2eb141164a401d89d3855d9d4309fb8f2aae3))
    - Allow `PackageSort` to be 'built' ([`334dfcb`](https://github.com/AOSC-Dev/oma-apt/commit/334dfcb1de31dd3f93af768ab43319db4b2f24b9))
    - Add more fields to the package object ([`d616a36`](https://github.com/AOSC-Dev/oma-apt/commit/d616a362bed637b6eadb25f92bc92f845fff30a0))
    - Change variable names ([`6b8a9a4`](https://github.com/AOSC-Dev/oma-apt/commit/6b8a9a43a876c9497a521912b8df0aeb0794e67e))
    - Store last lookup to prevent unnecessary lookups ([`c930a72`](https://github.com/AOSC-Dev/oma-apt/commit/c930a7226d04896effc0a20096ec10eaa1ec1737))
    - New `rustfmt.toml` ([`59b1144`](https://github.com/AOSC-Dev/oma-apt/commit/59b1144921f77870874d7f84b66c0e3513afba67))
    - Add more version fields ([`cd310e0`](https://github.com/AOSC-Dev/oma-apt/commit/cd310e06d067bdc587c255c421492dc91fad317f))
    - Add `summary` and `description` for versions ([`fcd4f97`](https://github.com/AOSC-Dev/oma-apt/commit/fcd4f97e4fa45a1ab6acf12860c840f844c1baca))
    - Change indentation to the superior tab ([`7103b4e`](https://github.com/AOSC-Dev/oma-apt/commit/7103b4e6fe1057ef172b1dff25af354d8efd56f7))
    - Comment unused functions ([`8496109`](https://github.com/AOSC-Dev/oma-apt/commit/8496109b71ea5653a71749f8779f6d002d2516fb))
    - Add `is_installed` on the version ([`7f30d9b`](https://github.com/AOSC-Dev/oma-apt/commit/7f30d9ba0127bf44cde80f5456608f25dd23f361))
    - Satisfy clippy and organize code ([`5ca9a50`](https://github.com/AOSC-Dev/oma-apt/commit/5ca9a50e7e3248e57c43aee465e56c5bc92b11e8))
    - Add simple test ([`6dbe4f7`](https://github.com/AOSC-Dev/oma-apt/commit/6dbe4f7a5edfd6b03749620848429b2bdd8b197c))
    - Use `Rc<RefCell>` for the records struct. ([`663ae5e`](https://github.com/AOSC-Dev/oma-apt/commit/663ae5e0d0bfe6c730d956b25b877c4588c55a5f))
    - Several memory leaks ([`c890491`](https://github.com/AOSC-Dev/oma-apt/commit/c89049197b6ce27c7e8b67d592c1a0e4ad9a92bf))
    - Give version structs their own pointer. ([`dfacd8e`](https://github.com/AOSC-Dev/oma-apt/commit/dfacd8edf7ef96072142b33293f8f7265516964b))
    - Rename types and use C++ Opaque Types ([`1ed41ae`](https://github.com/AOSC-Dev/oma-apt/commit/1ed41ae5b876f08ad8416c07f491d42a1966af64))
    - Initial fork and update to `rust-apt` ([`8834d20`](https://github.com/AOSC-Dev/oma-apt/commit/8834d20473101b593634047151eedeb324e3c6ab))
    - Clippy: unused import ([`a027786`](https://github.com/AOSC-Dev/oma-apt/commit/a0277868287910924a03ef80b611d0a23f6caf4f))
    - Cripple some examples on ye-old-apt ([`a603237`](https://github.com/AOSC-Dev/oma-apt/commit/a60323705ff67d59dc565be87cb12ac765828a88))
    - Missing header(?) ([`99fba90`](https://github.com/AOSC-Dev/oma-apt/commit/99fba9024ab7c0797ddb468e72c42dd612f28934))
    - Format ([`a5b8ede`](https://github.com/AOSC-Dev/oma-apt/commit/a5b8ede7ece2d783c8c9d0b8d56cfc3a2ff3883b))
    - Bump version to 0.3.2 ([`e073fe8`](https://github.com/AOSC-Dev/oma-apt/commit/e073fe8081cc0bfa44ffd775ac9e5dd26d1af651))
    - Add DepIterator to list dependencies of package versions ([`edc7008`](https://github.com/AOSC-Dev/oma-apt/commit/edc7008762056ac31c6f38fda350e6b5219b825d))
    - Expose 'sane' interface to external consumers ([`7b2d146`](https://github.com/AOSC-Dev/oma-apt/commit/7b2d146e8d77b483752f988a5e4137d7a083d578))
    - Add package detail accessors ([`07c92f0`](https://github.com/AOSC-Dev/oma-apt/commit/07c92f003b31e43440c1508733234b66cdd5d99a))
    - Add priority type accessor to VerView ([`1018536`](https://github.com/AOSC-Dev/oma-apt/commit/10185365236f1bf66eff5b7344f39b70f6a7b688))
    - Use correct raw type in PkgFileIterator ([`f28cd3e`](https://github.com/AOSC-Dev/oma-apt/commit/f28cd3e4997f12b0a6ad9d548a46a243b759d246))
    - Dev itertools -> 0.9 ([`52213cd`](https://github.com/AOSC-Dev/oma-apt/commit/52213cdd72b708323a2611ec926d372bd6b6d6df))
    - Bump travis ([`6d16f09`](https://github.com/AOSC-Dev/oma-apt/commit/6d16f0987fd3b1bdda47e2fda487518e8e065d6c))
    - Extra headers required on newer apt ([`ecd6090`](https://github.com/AOSC-Dev/oma-apt/commit/ecd60906d21b8bb65dd845b3e663053ae8098a49))
    - Work around https://github.com/rust-lang/rust/pull/57018 ([`3d66d10`](https://github.com/AOSC-Dev/oma-apt/commit/3d66d108b2aff2c273d9beb41daa8bc2031c1b3a))
    - Bump to xenial ([`f8c52cb`](https://github.com/AOSC-Dev/oma-apt/commit/f8c52cba964b172b0878a7eeb7b022f24f6e8bea))
    - Edition = 2018 ([`dac9411`](https://github.com/AOSC-Dev/oma-apt/commit/dac9411a5630c39fd2d758d96d308b476a40dcbe))
    - Minor lie in the readme ([`be7b547`](https://github.com/AOSC-Dev/oma-apt/commit/be7b5476234ddd34b481bc6188866be40defc90f))
    - Bump dev dependency / fmt ([`7f180bc`](https://github.com/AOSC-Dev/oma-apt/commit/7f180bc3efef303a6d65e9a546e64d447ecf3d0d))
    - Upgrade gcc crate to cc ([`46ad5e2`](https://github.com/AOSC-Dev/oma-apt/commit/46ad5e2bb2497c9d859bd771a9902a09e6652619))
    - New rustfmt ([`2f2167c`](https://github.com/AOSC-Dev/oma-apt/commit/2f2167c0276275c3261bf5d2dcb0f1d6a86bc384))
    - Test environment for building on trusty in docker ([`1d2364d`](https://github.com/AOSC-Dev/oma-apt/commit/1d2364d800dd16cfb6ef9a0f80328aff8e249ff9))
    - Support building with very old apt ([`46aa9ed`](https://github.com/AOSC-Dev/oma-apt/commit/46aa9ed07f76ebe4c50de14c4b35914b07b9edd7))
    - Update lazy-static to 1 ([`7a55e26`](https://github.com/AOSC-Dev/oma-apt/commit/7a55e2678bbf434bdec430798f5564107883029d))
    - Add a note about singleton breakage ([`3b7065e`](https://github.com/AOSC-Dev/oma-apt/commit/3b7065ed958bd3c41e2ba00c36863caa30ceb58f))
    - Remove empty test ([`c4e7590`](https://github.com/AOSC-Dev/oma-apt/commit/c4e7590239be504922b20b311744c61d915ccb75))
    - PhantomData cares about the MutexGuard, not the Cache ([`45741e2`](https://github.com/AOSC-Dev/oma-apt/commit/45741e2de1b058bcdf2e8d5715d4db0d715e2406))
    - Comments / unwrap->expect ([`e6f0bd8`](https://github.com/AOSC-Dev/oma-apt/commit/e6f0bd8b6b8b9120155d06d1ff39c1df89f913da))
    - Cache actually carries around a mutex ([`9ee8a3c`](https://github.com/AOSC-Dev/oma-apt/commit/9ee8a3ca09782496df68e72790cd9005cc7c81bc))
    - New rustfmt ([`e1cf0fa`](https://github.com/AOSC-Dev/oma-apt/commit/e1cf0fa26cecc60502553686d2c9042dbd6fd6a3))
    - Less specific version declarations ([`477038c`](https://github.com/AOSC-Dev/oma-apt/commit/477038c074b77b334eb664a8d3217ed740b79753))
    - Example of running on docker ([`6b2bd29`](https://github.com/AOSC-Dev/oma-apt/commit/6b2bd29fed3f4f04d4b60015cfbb65f469733fee))
    - Some extra iterator utilities ([`370a764`](https://github.com/AOSC-Dev/oma-apt/commit/370a76496730c1d8423bcd185d8d861b6b6f527f))
    - Thread safety note ([`db3be1e`](https://github.com/AOSC-Dev/oma-apt/commit/db3be1e3d276e3a916233ff0b027cfab2f4d8386))
    - Upgrade ([`0aa1d73`](https://github.com/AOSC-Dev/oma-apt/commit/0aa1d738c6c90b53385c0de4afcb826fd06b1571))
    - Try and convince travis to run in fast mode ([`8fd3d14`](https://github.com/AOSC-Dev/oma-apt/commit/8fd3d146e3e3f7cce59a3ed47be401c214bfd45e))
    - Cargo upgrade + fix deprecation warning ([`6bf783c`](https://github.com/AOSC-Dev/oma-apt/commit/6bf783c67b53a19b33a4ed93628e618a8b38bb56))
    - Version exposed ([`d13db34`](https://github.com/AOSC-Dev/oma-apt/commit/d13db3450bf76037e5e5a3bad0b4a9c20b57deb1))
    - Source-listing example sorts by version ([`f8d5c53`](https://github.com/AOSC-Dev/oma-apt/commit/f8d5c5370b83997a31ff1b006061ad20a66996b0))
    - Fmt ([`77918dc`](https://github.com/AOSC-Dev/oma-apt/commit/77918dcee785e196cd328d0e56c2639dc0d90476))
    - Expose version comparison ([`81b66ce`](https://github.com/AOSC-Dev/oma-apt/commit/81b66ce2ae696ba6a74d64f24cef09bd7f1561ba))
    - Fixed iterators, "origin"-like concept exposed ([`3314733`](https://github.com/AOSC-Dev/oma-apt/commit/3314733ebb4990d447cfb1132150fbfcb7db78c6))
    - Clippy ([`b1bb2ef`](https://github.com/AOSC-Dev/oma-apt/commit/b1bb2ef8cae8c7c19c0a193999367147b05e40ed))
    - Remove last warning with phantomdata ([`16aa763`](https://github.com/AOSC-Dev/oma-apt/commit/16aa7638452460de2cd698b109915a24f12b4b51))
    - Spello ([`6641457`](https://github.com/AOSC-Dev/oma-apt/commit/6641457af2586de2b64671835f0f85faf94d3e6f))
    - A port of list_source_packages.py, minus the cachedir changing ([`b2fc665`](https://github.com/AOSC-Dev/oma-apt/commit/b2fc66519b70d0b6382a3372915a3c60b072cef9))
    - Fmt ([`25c1de2`](https://github.com/AOSC-Dev/oma-apt/commit/25c1de2af2170685a7515f70ddc574e655dbe707))
    - A whole load of origin information may be absent; afaics only for /var/lib/dpkg/status? ([`c7b6207`](https://github.com/AOSC-Dev/oma-apt/commit/c7b6207ba60828b0b9590ddea28e0a93774659b1))
    - Surface a whole load more 'origin' information ([`e5a01ac`](https://github.com/AOSC-Dev/oma-apt/commit/e5a01acb71180d596a8c311029538e58c10ff6d7))
    - Fmt ([`4ced3f9`](https://github.com/AOSC-Dev/oma-apt/commit/4ced3f985cd42ecf9b7ce75faa0013b76c25fa67))
    - Some fiddling with the display ([`d935fa5`](https://github.com/AOSC-Dev/oma-apt/commit/d935fa5fb7678eeb698464d5f92113c703ef888d))
    - Impl Display; remove prettyprint for compat; fix section ([`457c31b`](https://github.com/AOSC-Dev/oma-apt/commit/457c31b4a4460d5bc55ceab46abece114ccefea8))
    - Thinking about other iterator things to add ([`d194842`](https://github.com/AOSC-Dev/oma-apt/commit/d1948421ceab3be4526c678e6673754f621fbc84))
    - Fixup borrows on nasty iterators ([`0018889`](https://github.com/AOSC-Dev/oma-apt/commit/00188891eb9d51abd8dff1d0cebe270ca2acbeeb))
    - Ask the 'gcc' crate to do more work, travis? ([`d4d85c2`](https://github.com/AOSC-Dev/oma-apt/commit/d4d85c248520ebed3c7141ce8c1eb3cca38fd1ed))
    - Travis' ubuntu is too old for c++0x, upgrade to trusty? ([`05aa328`](https://github.com/AOSC-Dev/oma-apt/commit/05aa328c6f6e04ecddc49cb2cc41ad2c67681610))
    - Install deps on travis ([`f277bb2`](https://github.com/AOSC-Dev/oma-apt/commit/f277bb22cf2369318e4cfc77090acc7b255ebf0b))
    - Fix module tests and write demonstration for the failure ([`cbc0f62`](https://github.com/AOSC-Dev/oma-apt/commit/cbc0f62106a9d11e2e4dfe70e623c7f8435bc0bd))
    - Fmt ([`6cde4ee`](https://github.com/AOSC-Dev/oma-apt/commit/6cde4ee61eecaa95a926798012f44476f39c3cc7))
    - Unify the Iterator implementations ([`d4dfd0f`](https://github.com/AOSC-Dev/oma-apt/commit/d4dfd0fb7af4802c548628a31909b64d6e24174a))
    - Docs / crates.io stuff ([`8f5bc80`](https://github.com/AOSC-Dev/oma-apt/commit/8f5bc80c06f8c7bfe5feb61eb2c8e488b57b38b4))
    - Cargo fmt ([`f4e4724`](https://github.com/AOSC-Dev/oma-apt/commit/f4e4724dd171f2a649a6a7492e7845c33068f964))
    - Clippy ([`6b164f1`](https://github.com/AOSC-Dev/oma-apt/commit/6b164f172dbc9510d1944b44dfc3711bd2cd99f1))
    - Fix warning with PhantomData ([`06a3c49`](https://github.com/AOSC-Dev/oma-apt/commit/06a3c497e7b41b15e3039b8a2e8818e09f3167fb))
    - Expose priority ([`70812eb`](https://github.com/AOSC-Dev/oma-apt/commit/70812eb615ab7f262776c254205f25ff6610b880))
    - Fix iterators with hack; fleshing out policy ([`67c2146`](https://github.com/AOSC-Dev/oma-apt/commit/67c2146b25ea1f71a121e1191962a7da341dde9e))
    - Versions; map misses the first item ([`d107ef3`](https://github.com/AOSC-Dev/oma-apt/commit/d107ef373f26e960d0b4c9e4cbaa15e80b3414b4))
    - Example listing ([`6acf62c`](https://github.com/AOSC-Dev/oma-apt/commit/6acf62ca687a977547767f160ac08033ede59091))
    - Switch to gcc build script ([`eaf79f4`](https://github.com/AOSC-Dev/oma-apt/commit/eaf79f46ed368370c28a58d0dafb45ade285d239))
    - Rename ([`0126dd5`](https://github.com/AOSC-Dev/oma-apt/commit/0126dd522078867b917c10fa8e36c62c94ebcb2b))
    - Give in and lazy_singleton the cache ([`aa46a93`](https://github.com/AOSC-Dev/oma-apt/commit/aa46a936f29f219fdff8a87f2d2a3307e1208726))
    - Find package by name ([`a394c4d`](https://github.com/AOSC-Dev/oma-apt/commit/a394c4d37f6debd7cba640dd5fb6b8bf2ebf1aa6))
    - Maybe expose some more things ([`c4655a5`](https://github.com/AOSC-Dev/oma-apt/commit/c4655a5bc51111bf4986a23b584027b1d83a6324))
    - Try and stop reinit segfaulting; doesn't really work ([`7562f18`](https://github.com/AOSC-Dev/oma-apt/commit/7562f18141bdac98c844e146da3e77ab658ffe61))
    - More natural interface; no idea why it needs that annotation ([`e90ee73`](https://github.com/AOSC-Dev/oma-apt/commit/e90ee73dc83b58720e3fec0cf16abec0cd65eeea))
    - Expose the iterator directly ([`e3f5bd3`](https://github.com/AOSC-Dev/oma-apt/commit/e3f5bd3633c4ee172a66b2c4af02b8f7b321d953))
    - This callback is dumb ([`4225e76`](https://github.com/AOSC-Dev/oma-apt/commit/4225e76641ea60c87856a0bc35cd9dde885f8976))
    - List packages (after manual make) ([`da91ba6`](https://github.com/AOSC-Dev/oma-apt/commit/da91ba690876c921d8c064c5b64dceedfaef6044))
    - Bindgen isn't even close to working; std::string is hard ([`39d8438`](https://github.com/AOSC-Dev/oma-apt/commit/39d8438d3a7ffebcd1c92a7743212747cd6fa478))
</details>

