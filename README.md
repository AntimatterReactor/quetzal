# Quetzal

<div align="center">
  <img src="https://img.shields.io/badge/status-under%20construction-yellow?style=for-the-badge" />
  <br/>
  <strong>⚠️ Quetzal is not ready for use. Expect breaking changes.</strong>
</div>

<br/>

The wise language.


> Not to be confused with the 90's Z-machine [_Quetzal File Format_](https://en.wikipedia.org/wiki/Quetzal_file_format),<br>
> nor to be confused with other quetzal projects, see [Disambiguation](#disambiguation).

## Background

Quetzal is a name for a handful of bird species from South America, also associated with Quetzalcoatl, the Aztec deity.

### Some History

I started this project, chosen this name approximately around the year of 2021.
I was a middle schooler back then, young and naive, but otherwise surprisingly good with programming.

Most of my current semi-obsession with language design stems from my first encounter with such arcane ideas, [Alex Apostolu's _Night_](https://github.com/alexapostolu/night).

## Getting Started

### Downloading

Use the following cargo command in your favorite shell:

```sh
$ cargo install quetzal-lang
```

Note that, for clarity sake: from this point on, the repo is named `quetzal`, the crate is called `quetzal-lang`, the library is called `libquetzal`, and the executable `quetzalc`.

### Building

```sh
$ git clone git@github.com:AntimatterReactor/quetzal.git
$ cd quetzal
$ cargo build
```

#### C++ and `cxx`

If you observe [Cargo.toml](Cargo.toml) and the codebase itself rather carefully, you might notice the gluing of c++ and rust together. This is done because llvm is built in c++. I don't use `inkwell` etc because it's lacking especially for JIT-compilation.

## An Overview

### Syntax

## Contributing

Any form of contribution is welcome. I might respond late quite often due to my timezone.

- You can check for bugs and report it in [Issues]().
- You can also check out the [todo list](TODO.md).
- Or you can even solve issues if there are any.

Contribution guideline coming soon! (or never, I dunno)

## Disambiguation

Quetzal, also known as `quetzalc` is not to be confused with:

- The [Z-Machine](https://en.wikipedia.org/wiki/Z-machine) [standardized save state file format](https://en.wikipedia.org/wiki/Quetzal_file_format) by Martin Frost 
- [Asriel](https://github.com/Asriel)'s abandoned [distributed hash table algorithms](https://crates.io/crates/quetzal).
- And the many many other projects such as:
    - [Quetzal-RDF's SQL translation engine](https://github.com/Quetzal-RDF/quetzal)
    - [systragroup's modeling library](https://github.com/QuetzalMX/QuetzalXLSReader)
    - etc...

## Contributing

If you're interested in contributing to further development of Quetzal, for which I salute you,
see [CONTRIBUTING.md](CONTRIBUTING.md)

## License

Except as otherwise noted, Quetzal is licensed under the Apache License, Version
2.0 [LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0> or the MIT
license [LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>, at your option.
