## Menu
- [Cargo plugins](#user-content-cargo-plugins) ([cargo-audit](#user-content-cargo-audit), [cargo-binutils](#user-content-cargo-binutils), [cargo-bom](#user-content-cargo-bom), [cargo-cache](#user-content-cargo-cache), [cargo-check](#user-content-cargo-check), [cargo-deb](#user-content-cargo-deb), [cargo-expand](#user-content-cargo-expand), [cargo-fuzz](#user-content-cargo-fuzz), [cargo-geiger](#user-content-cargo-geiger), [cargo-generate](#user-content-cargo-generate), [cargo-graph](#user-content-cargo-graph), [cargo-make](#user-content-cargo-make), [cargo-testify](#user-content-cargo-testify), [cargo-update](#user-content-cargo-update), [cargo-watch](#user-content-cargo-watch), )
- [Commandline libraries](#user-content-commandline-libraries) ([clap](#user-content-clap), )
- [Commandline tools](#user-content-commandline-tools) ([bat](#user-content-bat), [bindgen](#user-content-bindgen), [capnpc](#user-content-capnpc), [diesel_cli](#user-content-diesel_cli), [doxidize](#user-content-doxidize), [exa](#user-content-exa), [fd-find](#user-content-fd-find), [fselect](#user-content-fselect), [hyperfine](#user-content-hyperfine), [just](#user-content-just), [mdbook](#user-content-mdbook), [parallel](#user-content-parallel), [project_init](#user-content-project_init), [ripgrep](#user-content-ripgrep), [rustsym](#user-content-rustsym), [sccache](#user-content-sccache), [skim](#user-content-skim), [skim](#user-content-skim), [svd2rust](#user-content-svd2rust), [tin-summer](#user-content-tin-summer), [tokei](#user-content-tokei), [wasm-gc](#user-content-wasm-gc), [wasm-pack](#user-content-wasm-pack), [xargo](#user-content-xargo), )
- [Data persitance](#user-content-data-persitance) ([serde](#user-content-serde), [serde_yaml](#user-content-serde_yaml), )

## Cargo plugins
### <a name="cargo-audit"></a>cargo-audit [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-audit)
 <img src="https://img.shields.io/crates/d/cargo-audit.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-audit.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-audit.svg?style=plastic">

Audit Cargo.lock for crates with security vulnerabilities reported to the RustSec Advisory Database.
### <a name="cargo-binutils"></a>cargo-binutils [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-binutils) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/japaric/cargo-binutils)
 <img src="https://img.shields.io/github/last-commit/japaric/cargo-binutils.svg?style=plastic"> <img src="https://img.shields.io/github/tag/japaric/cargo-binutils.svg?style=plastic"> <img src="https://img.shields.io/crates/d/cargo-binutils.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-binutils.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-binutils.svg?style=plastic">

Cargo subcommands to invoke the LLVM tools shipped with the Rust toolchain
### <a name="cargo-bom"></a>cargo-bom [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-bom) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/sensorfu/cargo-bom)
 <img src="https://img.shields.io/github/last-commit/sensorfu/cargo-bom.svg?style=plastic"> <img src="https://img.shields.io/github/tag/sensorfu/cargo-bom.svg?style=plastic"> <img src="https://img.shields.io/crates/d/cargo-bom.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-bom.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-bom.svg?style=plastic">

This tool (cargo bom) can be used to construct Bill of Materials for software using Cargo package manager.
The output of cargo bom has two sections. First it prints out a table with all dependencies, version numbers and names of licenses. Then it prints all license texts found from depended projects (files matching glob "LICENSE*").
### <a name="cargo-cache"></a>cargo-cache [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-cache) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/matthiaskrgr/cargo-cache)
 <img src="https://img.shields.io/github/last-commit/matthiaskrgr/cargo-cache.svg?style=plastic"> <img src="https://img.shields.io/github/tag/matthiaskrgr/cargo-cache.svg?style=plastic"> <img src="https://img.shields.io/crates/d/cargo-cache.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-cache.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-cache.svg?style=plastic">

Display information on the cargo cache `~/.cargo/`. Optional cache pruning.
### <a name="cargo-check"></a>cargo-check [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-check)
 <img src="https://img.shields.io/crates/d/cargo-check.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-check.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-check.svg?style=plastic">

desc
### <a name="cargo-deb"></a>cargo-deb [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-deb)
 <img src="https://img.shields.io/crates/d/cargo-deb.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-deb.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-deb.svg?style=plastic">

desc
### <a name="cargo-expand"></a>cargo-expand [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-expand)
 <img src="https://img.shields.io/crates/d/cargo-expand.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-expand.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-expand.svg?style=plastic">

desc
### <a name="cargo-fuzz"></a>cargo-fuzz [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-fuzz)
 <img src="https://img.shields.io/crates/d/cargo-fuzz.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-fuzz.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-fuzz.svg?style=plastic">

desc
### <a name="cargo-geiger"></a>cargo-geiger [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-geiger)
 <img src="https://img.shields.io/crates/d/cargo-geiger.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-geiger.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-geiger.svg?style=plastic">

desc
### <a name="cargo-generate"></a>cargo-generate [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-generate)
 <img src="https://img.shields.io/crates/d/cargo-generate.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-generate.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-generate.svg?style=plastic">

desc
### <a name="cargo-graph"></a>cargo-graph [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-graph)
 <img src="https://img.shields.io/crates/d/cargo-graph.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-graph.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-graph.svg?style=plastic">

desc
### <a name="cargo-make"></a>cargo-make [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-make)
 <img src="https://img.shields.io/crates/d/cargo-make.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-make.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-make.svg?style=plastic">

desc
### <a name="cargo-testify"></a>cargo-testify [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-testify)
 <img src="https://img.shields.io/crates/d/cargo-testify.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-testify.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-testify.svg?style=plastic">

desc
### <a name="cargo-update"></a>cargo-update [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-update)
 <img src="https://img.shields.io/crates/d/cargo-update.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-update.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-update.svg?style=plastic">

desc
### <a name="cargo-watch"></a>cargo-watch [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-watch)
 <img src="https://img.shields.io/crates/d/cargo-watch.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-watch.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-watch.svg?style=plastic">

desc

## Commandline libraries
### <a name="clap"></a>clap [<img src="https://img.shields.io/badge/URL-homepage-navy.svg?style=plastic">](https://clap.rs) [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/clap) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/clap-rs/clap)
 <img src="https://img.shields.io/github/last-commit/clap-rs/clap.svg?style=plastic"> <img src="https://img.shields.io/github/tag/clap-rs/clap.svg?style=plastic"> <img src="https://img.shields.io/crates/d/clap.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/clap.svg?style=plastic"> <img src="https://img.shields.io/crates/l/clap.svg?style=plastic">

A full featured, fast Command Line Argument Parser for Rust

## Commandline tools
### <a name="bat"></a>bat [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/bat) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/sharkdp/bat)
 <img src="https://img.shields.io/github/last-commit/sharkdp/bat.svg?style=plastic"> <img src="https://img.shields.io/github/tag/sharkdp/bat.svg?style=plastic"> <img src="https://img.shields.io/crates/d/bat.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/bat.svg?style=plastic"> <img src="https://img.shields.io/crates/l/bat.svg?style=plastic">

A `cat(1)` clone with syntax highlighting and Git integration.
### <a name="bindgen"></a>bindgen [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/bindgen)
 <img src="https://img.shields.io/crates/d/bindgen.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/bindgen.svg?style=plastic"> <img src="https://img.shields.io/crates/l/bindgen.svg?style=plastic">

desc
### <a name="capnpc"></a>capnpc [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/capnpc)
 <img src="https://img.shields.io/crates/d/capnpc.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/capnpc.svg?style=plastic"> <img src="https://img.shields.io/crates/l/capnpc.svg?style=plastic">

desc
### <a name="diesel_cli"></a>diesel_cli [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/diesel_cli)
 <img src="https://img.shields.io/crates/d/diesel_cli.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/diesel_cli.svg?style=plastic"> <img src="https://img.shields.io/crates/l/diesel_cli.svg?style=plastic">

desc
### <a name="doxidize"></a>doxidize [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/doxidize)
 <img src="https://img.shields.io/crates/d/doxidize.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/doxidize.svg?style=plastic"> <img src="https://img.shields.io/crates/l/doxidize.svg?style=plastic">

desc
### <a name="exa"></a>exa [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/exa)
 <img src="https://img.shields.io/crates/d/exa.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/exa.svg?style=plastic"> <img src="https://img.shields.io/crates/l/exa.svg?style=plastic">

desc
### <a name="fd-find"></a>fd-find [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/fd-find)
 <img src="https://img.shields.io/crates/d/fd-find.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/fd-find.svg?style=plastic"> <img src="https://img.shields.io/crates/l/fd-find.svg?style=plastic">

desc
### <a name="fselect"></a>fselect [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/fselect)
 <img src="https://img.shields.io/crates/d/fselect.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/fselect.svg?style=plastic"> <img src="https://img.shields.io/crates/l/fselect.svg?style=plastic">

desc
### <a name="hyperfine"></a>hyperfine [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/hyperfine)
 <img src="https://img.shields.io/crates/d/hyperfine.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/hyperfine.svg?style=plastic"> <img src="https://img.shields.io/crates/l/hyperfine.svg?style=plastic">

desc
### <a name="just"></a>just [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/just)
 <img src="https://img.shields.io/crates/d/just.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/just.svg?style=plastic"> <img src="https://img.shields.io/crates/l/just.svg?style=plastic">

desc
### <a name="mdbook"></a>mdbook [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/mdbook)
 <img src="https://img.shields.io/crates/d/mdbook.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/mdbook.svg?style=plastic"> <img src="https://img.shields.io/crates/l/mdbook.svg?style=plastic">

desc
### <a name="parallel"></a>parallel [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/parallel)
 <img src="https://img.shields.io/crates/d/parallel.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/parallel.svg?style=plastic"> <img src="https://img.shields.io/crates/l/parallel.svg?style=plastic">

desc
### <a name="project_init"></a>project_init [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/project_init)
 <img src="https://img.shields.io/crates/d/project_init.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/project_init.svg?style=plastic"> <img src="https://img.shields.io/crates/l/project_init.svg?style=plastic">

desc
### <a name="ripgrep"></a>ripgrep [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/ripgrep)
 <img src="https://img.shields.io/crates/d/ripgrep.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/ripgrep.svg?style=plastic"> <img src="https://img.shields.io/crates/l/ripgrep.svg?style=plastic">

desc
### <a name="rustsym"></a>rustsym [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/rustsym)
 <img src="https://img.shields.io/crates/d/rustsym.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/rustsym.svg?style=plastic"> <img src="https://img.shields.io/crates/l/rustsym.svg?style=plastic">

desc
### <a name="sccache"></a>sccache [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/sccache)
 <img src="https://img.shields.io/crates/d/sccache.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/sccache.svg?style=plastic"> <img src="https://img.shields.io/crates/l/sccache.svg?style=plastic">

desc
### <a name="skim"></a>skim [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/skim)
 <img src="https://img.shields.io/crates/d/skim.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/skim.svg?style=plastic"> <img src="https://img.shields.io/crates/l/skim.svg?style=plastic">

desc
### <a name="skim"></a>skim [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/skim)
 <img src="https://img.shields.io/crates/d/skim.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/skim.svg?style=plastic"> <img src="https://img.shields.io/crates/l/skim.svg?style=plastic">

desc
### <a name="svd2rust"></a>svd2rust [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/svd2rust)
 <img src="https://img.shields.io/crates/d/svd2rust.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/svd2rust.svg?style=plastic"> <img src="https://img.shields.io/crates/l/svd2rust.svg?style=plastic">

desc
### <a name="tin-summer"></a>tin-summer [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/tin-summer)
 <img src="https://img.shields.io/crates/d/tin-summer.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/tin-summer.svg?style=plastic"> <img src="https://img.shields.io/crates/l/tin-summer.svg?style=plastic">

desc
### <a name="tokei"></a>tokei [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/tokei)
 <img src="https://img.shields.io/crates/d/tokei.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/tokei.svg?style=plastic"> <img src="https://img.shields.io/crates/l/tokei.svg?style=plastic">

desc
### <a name="wasm-gc"></a>wasm-gc [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/wasm-gc)
 <img src="https://img.shields.io/crates/d/wasm-gc.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/wasm-gc.svg?style=plastic"> <img src="https://img.shields.io/crates/l/wasm-gc.svg?style=plastic">

desc
### <a name="wasm-pack"></a>wasm-pack [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/wasm-pack)
 <img src="https://img.shields.io/crates/d/wasm-pack.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/wasm-pack.svg?style=plastic"> <img src="https://img.shields.io/crates/l/wasm-pack.svg?style=plastic">

desc
### <a name="xargo"></a>xargo [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/xargo)
 <img src="https://img.shields.io/crates/d/xargo.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/xargo.svg?style=plastic"> <img src="https://img.shields.io/crates/l/xargo.svg?style=plastic">

desc

## Data persitance
### <a name="serde"></a>serde [<img src="https://img.shields.io/badge/URL-homepage-navy.svg?style=plastic">](https://serde.rs/) [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/serde) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/serde-rs/serde)
 <img src="https://img.shields.io/github/last-commit/serde-rs/serde.svg?style=plastic"> <img src="https://img.shields.io/github/tag/serde-rs/serde.svg?style=plastic"> <img src="https://img.shields.io/crates/d/serde.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/serde.svg?style=plastic"> <img src="https://img.shields.io/crates/l/serde.svg?style=plastic">

Serde is a framework for serializing and deserializing Rust data structures efficiently and generically.
### <a name="serde_yaml"></a>serde_yaml [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/serde_yaml) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/dtolnay/serde-yaml)
 <img src="https://img.shields.io/github/last-commit/dtolnay/serde-yaml.svg?style=plastic"> <img src="https://img.shields.io/github/tag/dtolnay/serde-yaml.svg?style=plastic"> <img src="https://img.shields.io/crates/d/serde_yaml.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/serde_yaml.svg?style=plastic"> <img src="https://img.shields.io/crates/l/serde_yaml.svg?style=plastic">

Strongly typed YAML library for Rust
