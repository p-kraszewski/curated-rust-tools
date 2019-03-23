## Menu
- [Cargo plugins](#user-content-cargo-plugins) ([cargo-audit](#user-content-cargo-audit), [cargo-binutils](#user-content-cargo-binutils), [cargo-bom](#user-content-cargo-bom), [cargo-cache](#user-content-cargo-cache), [cargo-check](#user-content-cargo-check), [cargo-deb](#user-content-cargo-deb), [cargo-expand](#user-content-cargo-expand), [cargo-fuzz](#user-content-cargo-fuzz), [cargo-geiger](#user-content-cargo-geiger), [cargo-generate](#user-content-cargo-generate), [cargo-graph](#user-content-cargo-graph), [cargo-make](#user-content-cargo-make), [cargo-testify](#user-content-cargo-testify), [cargo-update](#user-content-cargo-update), [cargo-watch](#user-content-cargo-watch), )
- [Commandline libraries](#user-content-commandline-libraries) ([clap](#user-content-clap), )
- [Commandline tools](#user-content-commandline-tools) ([bat](#user-content-bat), [bindgen](#user-content-bindgen), [capnpc](#user-content-capnpc), [diesel_cli](#user-content-diesel_cli), [exa](#user-content-exa), [fd-find](#user-content-fd-find), [fselect](#user-content-fselect), [hyperfine](#user-content-hyperfine), [just](#user-content-just), [mdbook](#user-content-mdbook), [parallel](#user-content-parallel), [project_init](#user-content-project_init), [ripgrep](#user-content-ripgrep), [rustsym](#user-content-rustsym), [sccache](#user-content-sccache), [skim](#user-content-skim), [svd2rust](#user-content-svd2rust), [tin-summer](#user-content-tin-summer), [tokei](#user-content-tokei), [wasm-gc](#user-content-wasm-gc), [wasm-pack](#user-content-wasm-pack), [xargo](#user-content-xargo), )
- [Data persitance](#user-content-data-persitance) ([serde](#user-content-serde), [serde_yaml](#user-content-serde_yaml), )

## Cargo plugins
### <a name="cargo-audit"></a>cargo-audit [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-audit) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/rustsec/cargo-audit)
 <img src="https://img.shields.io/github/last-commit/rustsec/cargo-audit.svg?style=plastic"> <img src="https://img.shields.io/github/tag/rustsec/cargo-audit.svg?style=plastic"> <img src="https://img.shields.io/crates/d/cargo-audit.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-audit.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-audit.svg?style=plastic">

Audit Cargo.lock for crates with security vulnerabilities reported to the RustSec Advisory Database.
### <a name="cargo-binutils"></a>cargo-binutils [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-binutils) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/japaric/cargo-binutils)
 <img src="https://img.shields.io/github/last-commit/japaric/cargo-binutils.svg?style=plastic"> <img src="https://img.shields.io/github/tag/japaric/cargo-binutils.svg?style=plastic"> <img src="https://img.shields.io/crates/d/cargo-binutils.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-binutils.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-binutils.svg?style=plastic">

Cargo subcommands to invoke the LLVM tools shipped with the Rust toolchain
### <a name="cargo-bom"></a>cargo-bom [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-bom) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/sensorfu/cargo-bom)
 <img src="https://img.shields.io/github/last-commit/sensorfu/cargo-bom.svg?style=plastic"> <img src="https://img.shields.io/github/tag/sensorfu/cargo-bom.svg?style=plastic"> <img src="https://img.shields.io/crates/d/cargo-bom.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-bom.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-bom.svg?style=plastic">

This tool can be used to construct Bill of Materials for software using Cargo package manager.
The output of cargo bom has two sections. First it prints out a table with all dependencies, version numbers and names of licenses. Then it prints all license texts found from depended projects (files matching glob "LICENSE*").
### <a name="cargo-cache"></a>cargo-cache [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-cache) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/matthiaskrgr/cargo-cache)
 <img src="https://img.shields.io/github/last-commit/matthiaskrgr/cargo-cache.svg?style=plastic"> <img src="https://img.shields.io/github/tag/matthiaskrgr/cargo-cache.svg?style=plastic"> <img src="https://img.shields.io/crates/d/cargo-cache.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-cache.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-cache.svg?style=plastic">

Display information on the cargo cache `~/.cargo/`. Optional cache pruning.
### <a name="cargo-check"></a>cargo-check [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-check) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/rsolomo/cargo-check)
 <img src="https://img.shields.io/github/last-commit/rsolomo/cargo-check.svg?style=plastic"> <img src="https://img.shields.io/github/tag/rsolomo/cargo-check.svg?style=plastic"> <img src="https://img.shields.io/crates/d/cargo-check.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-check.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-check.svg?style=plastic">

This is a wrapper around `cargo rustc -- -Zno-trans`. It can be helpful for running a faster compile if you only need correctness checks.
### <a name="cargo-deb"></a>cargo-deb [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-deb) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/mmstick/cargo-deb) [<img src="https://img.shields.io/badge/URL-Docs.RS-navy.svg?style=plastic">](https://docs.rs/cargo-deb)
 <img src="https://img.shields.io/github/last-commit/mmstick/cargo-deb.svg?style=plastic"> <img src="https://img.shields.io/github/tag/mmstick/cargo-deb.svg?style=plastic"> <img src="https://img.shields.io/crates/d/cargo-deb.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-deb.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-deb.svg?style=plastic">

This is a Cargo helper command which automatically creates binary Debian packages (`.deb`) from Cargo projects.
### <a name="cargo-expand"></a>cargo-expand [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-expand) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/dtolnay/cargo-expand) <img src="https://img.shields.io/badge/Warning-Requires_nightly-red.svg?style=plastic">
 <img src="https://img.shields.io/github/last-commit/dtolnay/cargo-expand.svg?style=plastic"> <img src="https://img.shields.io/github/tag/dtolnay/cargo-expand.svg?style=plastic"> <img src="https://img.shields.io/crates/d/cargo-expand.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-expand.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-expand.svg?style=plastic">

Once installed, the following command prints out the result of macro expansion and `#[derive]` expansion applied to the current crate.
### <a name="cargo-fuzz"></a>cargo-fuzz [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-fuzz) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/rust-fuzz/cargo-fuzz)
 <img src="https://img.shields.io/github/last-commit/rust-fuzz/cargo-fuzz.svg?style=plastic"> <img src="https://img.shields.io/github/tag/rust-fuzz/cargo-fuzz.svg?style=plastic"> <img src="https://img.shields.io/crates/d/cargo-fuzz.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-fuzz.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-fuzz.svg?style=plastic">

Simple wrapper around libFuzzer
### <a name="cargo-geiger"></a>cargo-geiger [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-geiger) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/anderejd/cargo-geiger)
 <img src="https://img.shields.io/github/last-commit/anderejd/cargo-geiger.svg?style=plastic"> <img src="https://img.shields.io/github/tag/anderejd/cargo-geiger.svg?style=plastic"> <img src="https://img.shields.io/crates/d/cargo-geiger.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-geiger.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-geiger.svg?style=plastic">

A program that list statistics related to usage of `unsafe` Rust code in a Rust crate and all its dependencies.
### <a name="cargo-generate"></a>cargo-generate [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-generate) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/ashleygwilliams/cargo-generate)
 <img src="https://img.shields.io/github/last-commit/ashleygwilliams/cargo-generate.svg?style=plastic"> <img src="https://img.shields.io/github/tag/ashleygwilliams/cargo-generate.svg?style=plastic"> <img src="https://img.shields.io/crates/d/cargo-generate.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-generate.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-generate.svg?style=plastic">

The `cargo-generate` is a developer tool to help you get up and running quickly with a new Rust project by leveraging a pre-existing git repository as a template.
### <a name="cargo-graph"></a>cargo-graph [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-graph) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/kbknapp/cargo-graph)
 <img src="https://img.shields.io/github/last-commit/kbknapp/cargo-graph.svg?style=plastic"> <img src="https://img.shields.io/github/tag/kbknapp/cargo-graph.svg?style=plastic"> <img src="https://img.shields.io/crates/d/cargo-graph.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-graph.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-graph.svg?style=plastic">

A cargo subcommand for building GraphViz DOT files of dependency graphs.
### <a name="cargo-make"></a>cargo-make [<img src="https://img.shields.io/badge/URL-homepage-navy.svg?style=plastic">](https://sagiegurari.github.io/cargo-make) [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-make) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/sagiegurari/cargo-make)
 <img src="https://img.shields.io/github/last-commit/sagiegurari/cargo-make.svg?style=plastic"> <img src="https://img.shields.io/github/tag/sagiegurari/cargo-make.svg?style=plastic"> <img src="https://img.shields.io/crates/d/cargo-make.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-make.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-make.svg?style=plastic">

The cargo-make task runner enables to define and configure sets of tasks and run them as a flow.
### <a name="cargo-testify"></a>cargo-testify [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-testify) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/greyblake/cargo-testify) [<img src="https://img.shields.io/badge/URL-Docs.RS-navy.svg?style=plastic">](https://docs.rs/cargo-testify)
 <img src="https://img.shields.io/github/last-commit/greyblake/cargo-testify.svg?style=plastic"> <img src="https://img.shields.io/github/tag/greyblake/cargo-testify.svg?style=plastic"> <img src="https://img.shields.io/crates/d/cargo-testify.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-testify.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-testify.svg?style=plastic">

Automatically runs tests on your Rust project and notifies about the result.
### <a name="cargo-update"></a>cargo-update [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-update) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/nabijaczleweli/cargo-update)
 <img src="https://img.shields.io/github/last-commit/nabijaczleweli/cargo-update.svg?style=plastic"> <img src="https://img.shields.io/github/tag/nabijaczleweli/cargo-update.svg?style=plastic"> <img src="https://img.shields.io/crates/d/cargo-update.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-update.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-update.svg?style=plastic">

A `cargo` subcommand for checking and applying updates to installed executables
### <a name="cargo-watch"></a>cargo-watch [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/cargo-watch) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/passcod/cargo-watch) [<img src="https://img.shields.io/badge/URL-Docs.RS-navy.svg?style=plastic">](https://docs.rs/cargo-watch)
 <img src="https://img.shields.io/github/last-commit/passcod/cargo-watch.svg?style=plastic"> <img src="https://img.shields.io/github/tag/passcod/cargo-watch.svg?style=plastic"> <img src="https://img.shields.io/crates/d/cargo-watch.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/cargo-watch.svg?style=plastic"> <img src="https://img.shields.io/crates/l/cargo-watch.svg?style=plastic">

Cargo Watch watches over your project's source for changes, and runs Cargo commands when they occur.

## Commandline libraries
### <a name="clap"></a>clap [<img src="https://img.shields.io/badge/URL-homepage-navy.svg?style=plastic">](https://clap.rs) [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/clap) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/clap-rs/clap)
 <img src="https://img.shields.io/github/last-commit/clap-rs/clap.svg?style=plastic"> <img src="https://img.shields.io/github/tag/clap-rs/clap.svg?style=plastic"> <img src="https://img.shields.io/crates/d/clap.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/clap.svg?style=plastic"> <img src="https://img.shields.io/crates/l/clap.svg?style=plastic">

A full featured, fast Command Line Argument Parser for Rust

## Commandline tools
### <a name="bat"></a>bat [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/bat) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/sharkdp/bat)
 <img src="https://img.shields.io/github/last-commit/sharkdp/bat.svg?style=plastic"> <img src="https://img.shields.io/github/tag/sharkdp/bat.svg?style=plastic"> <img src="https://img.shields.io/crates/d/bat.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/bat.svg?style=plastic"> <img src="https://img.shields.io/crates/l/bat.svg?style=plastic">

A `cat(1)` clone with syntax highlighting and Git integration.
### <a name="bindgen"></a>bindgen [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/bindgen) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/rust-lang/rust-bindgen) [<img src="https://img.shields.io/badge/URL-Docs.RS-navy.svg?style=plastic">](https://docs.rs/bindgen)
 <img src="https://img.shields.io/github/last-commit/rust-lang/rust-bindgen.svg?style=plastic"> <img src="https://img.shields.io/github/tag/rust-lang/rust-bindgen.svg?style=plastic"> <img src="https://img.shields.io/crates/d/bindgen.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/bindgen.svg?style=plastic"> <img src="https://img.shields.io/crates/l/bindgen.svg?style=plastic">

bindgen automatically generates Rust FFI bindings to C (and some C++) libraries.
### <a name="capnpc"></a>capnpc [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/capnpc) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/capnproto/capnproto-rust) [<img src="https://img.shields.io/badge/URL-Docs.RS-navy.svg?style=plastic">](https://docs.rs/capnpc)
 <img src="https://img.shields.io/github/last-commit/capnproto/capnproto-rust.svg?style=plastic"> <img src="https://img.shields.io/github/tag/capnproto/capnproto-rust.svg?style=plastic"> <img src="https://img.shields.io/crates/d/capnpc.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/capnpc.svg?style=plastic"> <img src="https://img.shields.io/crates/l/capnpc.svg?style=plastic">

A `Cap'n Proto` code generation for Rust
### <a name="diesel_cli"></a>diesel_cli [<img src="https://img.shields.io/badge/URL-homepage-navy.svg?style=plastic">](https://diesel.rs) [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/diesel_cli) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/diesel-rs/diesel)
 <img src="https://img.shields.io/github/last-commit/diesel-rs/diesel.svg?style=plastic"> <img src="https://img.shields.io/github/tag/diesel-rs/diesel.svg?style=plastic"> <img src="https://img.shields.io/crates/d/diesel_cli.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/diesel_cli.svg?style=plastic"> <img src="https://img.shields.io/crates/l/diesel_cli.svg?style=plastic">

Diesel CLI is a tool that aids in managing your database schema. Migrations are bi-directional changes to your database that get applied sequentially.
### <a name="exa"></a>exa [<img src="https://img.shields.io/badge/URL-homepage-navy.svg?style=plastic">](https://the.exa.website/) [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/exa) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/ogham/exa)
 <img src="https://img.shields.io/github/last-commit/ogham/exa.svg?style=plastic"> <img src="https://img.shields.io/github/tag/ogham/exa.svg?style=plastic"> <img src="https://img.shields.io/crates/d/exa.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/exa.svg?style=plastic"> <img src="https://img.shields.io/crates/l/exa.svg?style=plastic">

exa is a replacement for `ls` written in Rust.
### <a name="fd-find"></a>fd-find [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/fd-find) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/sharkdp/fd)
 <img src="https://img.shields.io/github/last-commit/sharkdp/fd.svg?style=plastic"> <img src="https://img.shields.io/github/tag/sharkdp/fd.svg?style=plastic"> <img src="https://img.shields.io/crates/d/fd-find.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/fd-find.svg?style=plastic"> <img src="https://img.shields.io/crates/l/fd-find.svg?style=plastic">

fd is a simple, fast and user-friendly alternative to `find`.
### <a name="fselect"></a>fselect [<img src="https://img.shields.io/badge/URL-homepage-navy.svg?style=plastic">](https://github.com/jhspetersson/fselect/blob/master/docs/usage.md) [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/fselect) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/jhspetersson/fselect)
 <img src="https://img.shields.io/github/last-commit/jhspetersson/fselect.svg?style=plastic"> <img src="https://img.shields.io/github/tag/jhspetersson/fselect.svg?style=plastic"> <img src="https://img.shields.io/crates/d/fselect.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/fselect.svg?style=plastic"> <img src="https://img.shields.io/crates/l/fselect.svg?style=plastic">

Find files with SQL-like queries
### <a name="hyperfine"></a>hyperfine [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/hyperfine) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/sharkdp/hyperfine)
 <img src="https://img.shields.io/github/last-commit/sharkdp/hyperfine.svg?style=plastic"> <img src="https://img.shields.io/github/tag/sharkdp/hyperfine.svg?style=plastic"> <img src="https://img.shields.io/crates/d/hyperfine.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/hyperfine.svg?style=plastic"> <img src="https://img.shields.io/crates/l/hyperfine.svg?style=plastic">

A command-line benchmarking tool (inspired by [bench](https://github.com/Gabriel439/bench)).
### <a name="just"></a>just [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/just) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/casey/just) [<img src="https://img.shields.io/badge/URL-Docs.RS-navy.svg?style=plastic">](https://docs.rs/just)
 <img src="https://img.shields.io/github/last-commit/casey/just.svg?style=plastic"> <img src="https://img.shields.io/github/tag/casey/just.svg?style=plastic"> <img src="https://img.shields.io/crates/d/just.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/just.svg?style=plastic"> <img src="https://img.shields.io/crates/l/just.svg?style=plastic">

just is a handy way to save and run project-specific commands (similar to `make`).
### <a name="mdbook"></a>mdbook [<img src="https://img.shields.io/badge/URL-homepage-navy.svg?style=plastic">](https://rust-lang-nursery.github.io/mdBook/index.html) [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/mdbook) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/rust-lang-nursery/mdBook)
 <img src="https://img.shields.io/github/last-commit/rust-lang-nursery/mdBook.svg?style=plastic"> <img src="https://img.shields.io/github/tag/rust-lang-nursery/mdBook.svg?style=plastic"> <img src="https://img.shields.io/crates/d/mdbook.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/mdbook.svg?style=plastic"> <img src="https://img.shields.io/crates/l/mdbook.svg?style=plastic">

mdBook is a utility to create modern online books from Markdown files.
### <a name="parallel"></a>parallel [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/parallel) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/mmstick/parallel)
 <img src="https://img.shields.io/github/last-commit/mmstick/parallel.svg?style=plastic"> <img src="https://img.shields.io/github/tag/mmstick/parallel.svg?style=plastic"> <img src="https://img.shields.io/crates/d/parallel.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/parallel.svg?style=plastic"> <img src="https://img.shields.io/crates/l/parallel.svg?style=plastic">

This is an attempt at recreating the functionality of [GNU Parallel](https://www.gnu.org/software/parallel/), a work-stealer for the command-line, in Rust under a MIT license.
### <a name="project_init"></a>project_init [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/project_init) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/vmchale/project-init) <img src="https://img.shields.io/badge/Warning-Requires_nightly-red.svg?style=plastic">
 <img src="https://img.shields.io/github/last-commit/vmchale/project-init.svg?style=plastic"> <img src="https://img.shields.io/github/tag/vmchale/project-init.svg?style=plastic"> <img src="https://img.shields.io/crates/d/project_init.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/project_init.svg?style=plastic"> <img src="https://img.shields.io/crates/l/project_init.svg?style=plastic">

pi is a command-line utility to initialize projects. It is written in rust.
### <a name="ripgrep"></a>ripgrep [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/ripgrep) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/BurntSushi/ripgrep)
 <img src="https://img.shields.io/github/last-commit/BurntSushi/ripgrep.svg?style=plastic"> <img src="https://img.shields.io/github/tag/BurntSushi/ripgrep.svg?style=plastic"> <img src="https://img.shields.io/crates/d/ripgrep.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/ripgrep.svg?style=plastic"> <img src="https://img.shields.io/crates/l/ripgrep.svg?style=plastic">

ripgrep is a line-oriented search tool that recursively searches your current directory for a regex pattern while respecting your gitignore rules.
### <a name="rustsym"></a>rustsym [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/rustsym) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/trixnz/rustsym)
 <img src="https://img.shields.io/github/last-commit/trixnz/rustsym.svg?style=plastic"> <img src="https://img.shields.io/github/tag/trixnz/rustsym.svg?style=plastic"> <img src="https://img.shields.io/crates/d/rustsym.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/rustsym.svg?style=plastic"> <img src="https://img.shields.io/crates/l/rustsym.svg?style=plastic">

A tool to query symbols from rust code for use in IDEs
### <a name="sccache"></a>sccache [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/sccache) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/mozilla/sccache) [<img src="https://img.shields.io/badge/URL-Docs.RS-navy.svg?style=plastic">](https://docs.rs/sccache)
 <img src="https://img.shields.io/github/last-commit/mozilla/sccache.svg?style=plastic"> <img src="https://img.shields.io/github/tag/mozilla/sccache.svg?style=plastic"> <img src="https://img.shields.io/crates/d/sccache.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/sccache.svg?style=plastic"> <img src="https://img.shields.io/crates/l/sccache.svg?style=plastic">

Sccache is a `ccache`-like tool. It is used as a compiler wrapper and avoids compilation when possible, storing a cache in a remote storage using the Amazon Simple Cloud Storage Service (S3) API, the Google Cloud Storage (GCS) API, or Redis.
### <a name="skim"></a>skim [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/skim) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/lotabout/skim)
 <img src="https://img.shields.io/github/last-commit/lotabout/skim.svg?style=plastic"> <img src="https://img.shields.io/github/tag/lotabout/skim.svg?style=plastic"> <img src="https://img.shields.io/crates/d/skim.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/skim.svg?style=plastic"> <img src="https://img.shields.io/crates/l/skim.svg?style=plastic">

Half of our life is spent on navigation; files, lines, commands… You need skim! It is a general fuzzy finder that saves you time.
### <a name="svd2rust"></a>svd2rust [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/svd2rust) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/japaric/svd2rust)
 <img src="https://img.shields.io/github/last-commit/japaric/svd2rust.svg?style=plastic"> <img src="https://img.shields.io/github/tag/japaric/svd2rust.svg?style=plastic"> <img src="https://img.shields.io/crates/d/svd2rust.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/svd2rust.svg?style=plastic"> <img src="https://img.shields.io/crates/l/svd2rust.svg?style=plastic">

Generate Rust register maps (`struct`s) from SVD files. Essential for any ARM Cortex-M programmers ou there.
### <a name="tin-summer"></a>tin-summer [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/tin-summer) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/vmchale/tin-summer)
 <img src="https://img.shields.io/github/last-commit/vmchale/tin-summer.svg?style=plastic"> <img src="https://img.shields.io/github/tag/vmchale/tin-summer.svg?style=plastic"> <img src="https://img.shields.io/crates/d/tin-summer.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/tin-summer.svg?style=plastic"> <img src="https://img.shields.io/crates/l/tin-summer.svg?style=plastic">

If you do a significant amount of programming, you'll probably end up with build artifacts scattered about. sn is a tool to help you find those artifacts.
`sn` is also a replacement for du. It has nicer output, saner commands and defaults, and it even runs faster on big directories thanks to multithreading.
### <a name="tokei"></a>tokei [<img src="https://img.shields.io/badge/URL-homepage-navy.svg?style=plastic">](https://tokei.rs/) [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/tokei) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/Aaronepower/tokei) [<img src="https://img.shields.io/badge/URL-Docs.RS-navy.svg?style=plastic">](https://docs.rs/tokei)
 <img src="https://img.shields.io/github/last-commit/Aaronepower/tokei.svg?style=plastic"> <img src="https://img.shields.io/github/tag/Aaronepower/tokei.svg?style=plastic"> <img src="https://img.shields.io/crates/d/tokei.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/tokei.svg?style=plastic"> <img src="https://img.shields.io/crates/l/tokei.svg?style=plastic">

Tokei is a program that displays statistics about your code. Tokei will show number of files, total lines within those files and code, comments, and blanks grouped by language.
### <a name="wasm-gc"></a>wasm-gc [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/wasm-gc) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/alexcrichton/wasm-gc)
 <img src="https://img.shields.io/github/last-commit/alexcrichton/wasm-gc.svg?style=plastic"> <img src="https://img.shields.io/github/tag/alexcrichton/wasm-gc.svg?style=plastic"> <img src="https://img.shields.io/crates/d/wasm-gc.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/wasm-gc.svg?style=plastic"> <img src="https://img.shields.io/crates/l/wasm-gc.svg?style=plastic">

A small command to gc a wasm module and remove all unneeded exports, imports, functions, etc. This is effectively `--gc-sections` for `wasm`.
### <a name="wasm-pack"></a>wasm-pack [<img src="https://img.shields.io/badge/URL-homepage-navy.svg?style=plastic">](https://rustwasm.github.io/wasm-pack/) [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/wasm-pack) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/ashleygwilliams/wasm-pack)
 <img src="https://img.shields.io/github/last-commit/ashleygwilliams/wasm-pack.svg?style=plastic"> <img src="https://img.shields.io/github/tag/ashleygwilliams/wasm-pack.svg?style=plastic"> <img src="https://img.shields.io/crates/d/wasm-pack.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/wasm-pack.svg?style=plastic"> <img src="https://img.shields.io/crates/l/wasm-pack.svg?style=plastic">

This tool seeks to be a one-stop shop for building and working with rust- generated WebAssembly that you would like to interop with JavaScript, in the browser or with Node.js. wasm-pack helps you build rust-generated WebAssembly packages that you could publish to the npm registry, or otherwise use alongside any javascript packages in workflows that you already use, such as webpack or greenkeeper.
### <a name="xargo"></a>xargo [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/xargo) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/japaric/xargo)
 <img src="https://img.shields.io/github/last-commit/japaric/xargo.svg?style=plastic"> <img src="https://img.shields.io/github/tag/japaric/xargo.svg?style=plastic"> <img src="https://img.shields.io/crates/d/xargo.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/xargo.svg?style=plastic"> <img src="https://img.shields.io/crates/l/xargo.svg?style=plastic">

Cross-cargo, the sysroot manager that lets you build and customize `std`

## Data persitance
### <a name="serde"></a>serde [<img src="https://img.shields.io/badge/URL-homepage-navy.svg?style=plastic">](https://serde.rs/) [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/serde) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/serde-rs/serde) [<img src="https://img.shields.io/badge/URL-Docs.RS-navy.svg?style=plastic">](https://docs.rs/serde)
 <img src="https://img.shields.io/github/last-commit/serde-rs/serde.svg?style=plastic"> <img src="https://img.shields.io/github/tag/serde-rs/serde.svg?style=plastic"> <img src="https://img.shields.io/crates/d/serde.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/serde.svg?style=plastic"> <img src="https://img.shields.io/crates/l/serde.svg?style=plastic">

Serde is a framework for serializing and deserializing Rust data structures efficiently and generically.
### <a name="serde_yaml"></a>serde_yaml [<img src="https://img.shields.io/badge/URL-Crates.IO-navy.svg?style=plastic">](https://crates.io/crates/serde_yaml) [<img src="https://img.shields.io/badge/URL-GitHub-navy.svg?style=plastic">](https://github.com/dtolnay/serde-yaml) [<img src="https://img.shields.io/badge/URL-Docs.RS-navy.svg?style=plastic">](https://docs.rs/serde_yaml)
 <img src="https://img.shields.io/github/last-commit/dtolnay/serde-yaml.svg?style=plastic"> <img src="https://img.shields.io/github/tag/dtolnay/serde-yaml.svg?style=plastic"> <img src="https://img.shields.io/crates/d/serde_yaml.svg?style=plastic"> <img src="https://img.shields.io/crates/dv/serde_yaml.svg?style=plastic"> <img src="https://img.shields.io/crates/l/serde_yaml.svg?style=plastic">

Strongly typed YAML library for Rust
