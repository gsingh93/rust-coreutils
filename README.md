Rust Coreutils
==============

This repository contains implementations of various tools from the [Coreutils](https://www.gnu.org/software/coreutils/) package written in the [Rust](http://www.rust-lang.org/) programming language.

These implementations are currently not POSIX-compliant and should not be used if you require POSIX-compliant behavior.

Pull requests adding tools, fixing bugs, or improving POSIX-compliance are welcome.

To build, run `make` in the root directory. All programs in `src` will be compiled and the binaries will be placed in `bin`.
