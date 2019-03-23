## Menu
- [Cargo plugins](#cargo-plugins) ([cargo-audit](#cargo-audit), [cargo-binutils](#cargo-binutils), [cargo-bom](#cargo-bom), [cargo-cache](#cargo-cache), [cargo-check](#cargo-check), [cargo-deb](#cargo-deb), [cargo-expand](#cargo-expand), [cargo-fuzz](#cargo-fuzz), [cargo-geiger](#cargo-geiger), [cargo-generate](#cargo-generate), [cargo-graph](#cargo-graph), [cargo-make](#cargo-make), [cargo-testify](#cargo-testify), [cargo-update](#cargo-update), [cargo-watch](#cargo-watch), )
- [Commandline libraries](#commandline-libraries) ([clap](#clap), )
- [Commandline tools](#commandline-tools) ([bat](#bat), [bindgen](#bindgen), [capnpc](#capnpc), [diesel_cli](#diesel_cli), [doxidize](#doxidize), [exa](#exa), [fd-find](#fd-find), [fselect](#fselect), [hyperfine](#hyperfine), [just](#just), [mdbook](#mdbook), [parallel](#parallel), [project_init](#project_init), [ripgrep](#ripgrep), [rustsym](#rustsym), [sccache](#sccache), [skim](#skim), [skim](#skim), [svd2rust](#svd2rust), [tin-summer](#tin-summer), [tokei](#tokei), [wasm-gc](#wasm-gc), [wasm-pack](#wasm-pack), [xargo](#xargo), )
- [Data persitance](#data-persitance) ([serde](#serde), [serde_yaml](#serde_yaml), )

## Cargo plugins
### cargo-audit [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-audit)
 <img src="https://img.shields.io/crates/d/cargo-audit.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-audit.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-audit.svg?style=plastic">

Audit Cargo.lock for crates with security vulnerabilities reported to the RustSec Advisory Database.
### cargo-binutils [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-binutils) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/japaric/cargo-binutils)
 <img src="https://img.shields.io/github/last-commit/japaric/cargo-binutils.svg?style=plastic"> <img src="https://img.shields.io/github/tag/japaric/cargo-binutils.svg?style=plastic"> <img src="https://img.shields.io/crates/d/cargo-binutils.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-binutils.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-binutils.svg?style=plastic">

Cargo subcommands to invoke the LLVM tools shipped with the Rust toolchain
### cargo-bom [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-bom) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/sensorfu/cargo-bom)
 <img src="https://img.shields.io/github/last-commit/sensorfu/cargo-bom.svg?style=plastic"> <img src="https://img.shields.io/github/tag/sensorfu/cargo-bom.svg?style=plastic"> <img src="https://img.shields.io/crates/d/cargo-bom.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-bom.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-bom.svg?style=plastic">

This tool (cargo bom) can be used to construct Bill of Materials for software using Cargo package manager.
The output of cargo bom has two sections. First it prints out a table with all dependencies, version numbers and names of licenses. Then it prints all license texts found from depended projects (files matching glob "LICENSE*").
### cargo-cache [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-cache) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/matthiaskrgr/cargo-cache)
 <img src="https://img.shields.io/github/last-commit/matthiaskrgr/cargo-cache.svg?style=plastic"> <img src="https://img.shields.io/github/tag/matthiaskrgr/cargo-cache.svg?style=plastic"> <img src="https://img.shields.io/crates/d/cargo-cache.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-cache.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-cache.svg?style=plastic">

Display information on the cargo cache `~/.cargo/`. Optional cache pruning.
### cargo-check [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-check)
 <img src="https://img.shields.io/crates/d/cargo-check.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-check.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-check.svg?style=plastic">

desc
### cargo-deb [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-deb)
 <img src="https://img.shields.io/crates/d/cargo-deb.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-deb.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-deb.svg?style=plastic">

desc
### cargo-expand [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-expand)
 <img src="https://img.shields.io/crates/d/cargo-expand.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-expand.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-expand.svg?style=plastic">

desc
### cargo-fuzz [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-fuzz)
 <img src="https://img.shields.io/crates/d/cargo-fuzz.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-fuzz.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-fuzz.svg?style=plastic">

desc
### cargo-geiger [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-geiger)
 <img src="https://img.shields.io/crates/d/cargo-geiger.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-geiger.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-geiger.svg?style=plastic">

desc
### cargo-generate [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-generate)
 <img src="https://img.shields.io/crates/d/cargo-generate.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-generate.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-generate.svg?style=plastic">

desc
### cargo-graph [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-graph)
 <img src="https://img.shields.io/crates/d/cargo-graph.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-graph.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-graph.svg?style=plastic">

desc
### cargo-make [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-make)
 <img src="https://img.shields.io/crates/d/cargo-make.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-make.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-make.svg?style=plastic">

desc
### cargo-testify [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-testify)
 <img src="https://img.shields.io/crates/d/cargo-testify.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-testify.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-testify.svg?style=plastic">

desc
### cargo-update [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-update)
 <img src="https://img.shields.io/crates/d/cargo-update.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-update.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-update.svg?style=plastic">

desc
### cargo-watch [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-watch)
 <img src="https://img.shields.io/crates/d/cargo-watch.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-watch.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-watch.svg?style=plastic">

desc

## Commandline libraries
### clap [<img src="https://img.shields.io/badge/URL-homepage-navy.svg?style=plastic">](https://clap.rs) [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/clap) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/clap-rs/clap)
 <img src="https://img.shields.io/github/last-commit/clap-rs/clap.svg?style=plastic"> <img src="https://img.shields.io/github/tag/clap-rs/clap.svg?style=plastic"> <img src="https://img.shields.io/crates/d/clap.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/clap.svg?style=plastic"> <img src="https://img.shields.io/crates/l/clap.svg?style=plastic">

A full featured, fast Command Line Argument Parser for Rust

## Commandline tools
### bat [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/bat) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/sharkdp/bat)
 <img src="https://img.shields.io/github/last-commit/sharkdp/bat.svg?style=plastic"> <img src="https://img.shields.io/github/tag/sharkdp/bat.svg?style=plastic"> <img src="https://img.shields.io/crates/d/bat.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/bat.svg?style=plastic"> <img src="https://img.shields.io/crates/l/bat.svg?style=plastic">

A `cat(1)` clone with syntax highlighting and Git integration.
### bindgen [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/bindgen)
 <img src="https://img.shields.io/crates/d/bindgen.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/bindgen.svg?style=plastic"> <img src="https://img.shields.io/crates/l/bindgen.svg?style=plastic">

desc
### capnpc [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/capnpc)
 <img src="https://img.shields.io/crates/d/capnpc.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/capnpc.svg?style=plastic"> <img src="https://img.shields.io/crates/l/capnpc.svg?style=plastic">

desc
### diesel_cli [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/diesel_cli)
 <img src="https://img.shields.io/crates/d/diesel_cli.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/diesel_cli.svg?style=plastic"> <img src="https://img.shields.io/crates/l/diesel_cli.svg?style=plastic">

desc
### doxidize [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/doxidize)
 <img src="https://img.shields.io/crates/d/doxidize.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/doxidize.svg?style=plastic"> <img src="https://img.shields.io/crates/l/doxidize.svg?style=plastic">

desc
### exa [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/exa)
 <img src="https://img.shields.io/crates/d/exa.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/exa.svg?style=plastic"> <img src="https://img.shields.io/crates/l/exa.svg?style=plastic">

desc
### fd-find [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/fd-find)
 <img src="https://img.shields.io/crates/d/fd-find.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/fd-find.svg?style=plastic"> <img src="https://img.shields.io/crates/l/fd-find.svg?style=plastic">

desc
### fselect [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/fselect)
 <img src="https://img.shields.io/crates/d/fselect.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/fselect.svg?style=plastic"> <img src="https://img.shields.io/crates/l/fselect.svg?style=plastic">

desc
### hyperfine [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/hyperfine)
 <img src="https://img.shields.io/crates/d/hyperfine.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/hyperfine.svg?style=plastic"> <img src="https://img.shields.io/crates/l/hyperfine.svg?style=plastic">

desc
### just [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/just)
 <img src="https://img.shields.io/crates/d/just.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/just.svg?style=plastic"> <img src="https://img.shields.io/crates/l/just.svg?style=plastic">

desc
### mdbook [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/mdbook)
 <img src="https://img.shields.io/crates/d/mdbook.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/mdbook.svg?style=plastic"> <img src="https://img.shields.io/crates/l/mdbook.svg?style=plastic">

desc
### parallel [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/parallel)
 <img src="https://img.shields.io/crates/d/parallel.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/parallel.svg?style=plastic"> <img src="https://img.shields.io/crates/l/parallel.svg?style=plastic">

desc
### project_init [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/project_init)
 <img src="https://img.shields.io/crates/d/project_init.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/project_init.svg?style=plastic"> <img src="https://img.shields.io/crates/l/project_init.svg?style=plastic">

desc
### ripgrep [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/ripgrep)
 <img src="https://img.shields.io/crates/d/ripgrep.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/ripgrep.svg?style=plastic"> <img src="https://img.shields.io/crates/l/ripgrep.svg?style=plastic">

desc
### rustsym [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/rustsym)
 <img src="https://img.shields.io/crates/d/rustsym.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/rustsym.svg?style=plastic"> <img src="https://img.shields.io/crates/l/rustsym.svg?style=plastic">

desc
### sccache [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/sccache)
 <img src="https://img.shields.io/crates/d/sccache.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/sccache.svg?style=plastic"> <img src="https://img.shields.io/crates/l/sccache.svg?style=plastic">

desc
### skim [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/skim)
 <img src="https://img.shields.io/crates/d/skim.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/skim.svg?style=plastic"> <img src="https://img.shields.io/crates/l/skim.svg?style=plastic">

desc
### skim [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/skim)
 <img src="https://img.shields.io/crates/d/skim.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/skim.svg?style=plastic"> <img src="https://img.shields.io/crates/l/skim.svg?style=plastic">

desc
### svd2rust [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/svd2rust)
 <img src="https://img.shields.io/crates/d/svd2rust.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/svd2rust.svg?style=plastic"> <img src="https://img.shields.io/crates/l/svd2rust.svg?style=plastic">

desc
### tin-summer [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/tin-summer)
 <img src="https://img.shields.io/crates/d/tin-summer.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/tin-summer.svg?style=plastic"> <img src="https://img.shields.io/crates/l/tin-summer.svg?style=plastic">

desc
### tokei [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/tokei)
 <img src="https://img.shields.io/crates/d/tokei.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/tokei.svg?style=plastic"> <img src="https://img.shields.io/crates/l/tokei.svg?style=plastic">

desc
### wasm-gc [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/wasm-gc)
 <img src="https://img.shields.io/crates/d/wasm-gc.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/wasm-gc.svg?style=plastic"> <img src="https://img.shields.io/crates/l/wasm-gc.svg?style=plastic">

desc
### wasm-pack [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/wasm-pack)
 <img src="https://img.shields.io/crates/d/wasm-pack.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/wasm-pack.svg?style=plastic"> <img src="https://img.shields.io/crates/l/wasm-pack.svg?style=plastic">

desc
### xargo [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/xargo)
 <img src="https://img.shields.io/crates/d/xargo.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/xargo.svg?style=plastic"> <img src="https://img.shields.io/crates/l/xargo.svg?style=plastic">

desc

## Data persitance
### serde [<img src="https://img.shields.io/badge/URL-homepage-navy.svg?style=plastic">](https://serde.rs/) [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/serde) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/serde-rs/serde)
 <img src="https://img.shields.io/github/last-commit/serde-rs/serde.svg?style=plastic"> <img src="https://img.shields.io/github/tag/serde-rs/serde.svg?style=plastic"> <img src="https://img.shields.io/crates/d/serde.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/serde.svg?style=plastic"> <img src="https://img.shields.io/crates/l/serde.svg?style=plastic">

Serde is a framework for serializing and deserializing Rust data structures efficiently and generically.
### serde_yaml [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/serde_yaml) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/dtolnay/serde-yaml)
 <img src="https://img.shields.io/github/last-commit/dtolnay/serde-yaml.svg?style=plastic"> <img src="https://img.shields.io/github/tag/dtolnay/serde-yaml.svg?style=plastic"> <img src="https://img.shields.io/crates/d/serde_yaml.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/serde_yaml.svg?style=plastic"> <img src="https://img.shields.io/crates/l/serde_yaml.svg?style=plastic">

Strongly typed YAML library for Rust
