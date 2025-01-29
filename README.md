# funguild

*Fetch the functional guild of a taxon based on FunGuild data*

<a href="https://github.com/Ebedthan/funguild/blob/main/LICENSE-MIT">
    <img src="https://img.shields.io/badge/license-MIT-blue?style=flat">
</a>
<a href="https://github.com/Ebedthan/funguild/blob/main/LICENSE-APACHE">
    <img src="https://img.shields.io/badge/license-APACHE-blue?style=flat">
</a>

## 🗺️ Overview

`funguild` is a [Rust](https://www.rust-lang.org) tool that fetches the functional guild of a fungi taxon based on FunGuild database.
It provides a simple and intuitive tool to easily retrive this data.

## 🔧 Installing

### From source

#### Download source using Git

```
git clone https://github.com/Ebedthan/funguild.git
```

#### Installing

You can install using [cargo](https://doc.rust-lang.org/cargo/):

```
# Change "funguild" to correct folder name
cd funguild

# If default rust install directory is ~/.cargo
cargo install --path . --root ~/.cargo
funguild -h
```


## ⚠️ Issue Tracker

Found a bug ? Have an enhancement request ? Head over to the [GitHub issue tracker](https://github.com/Ebedthan/funguild/issues) if you need to report or ask something.
If you are filing in on a bug, please include as much information as you can about the issue, and try to recreate the same bug in a simple, easily reproducible situation.

## ⚖️ License

`funguild` is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](https://github.com/Ebedthan/funguild/blob/main/LICENSE-APACHE) and [LICENSE-MIT](https://github.com/Ebedthan/funguild/blob/main/LICENSE-MIT) for details.

Full help is available from `funguild --help`.

## Minimum supported Rust version
`funguild` minimum [Rust](https://www.rust-lang.org/) version is 1.74.1.

## Semver
`funguild` is following [Semantic Versioning 2.0](https://semver.org/).
