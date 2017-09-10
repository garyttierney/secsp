# secsp

[![Coverage Status](https://coveralls.io/repos/github/garyttierney/rust-csp/badge.svg?branch=master)](https://coveralls.io/github/garyttierney/rust-csp?branch=master)

[![Build Status](https://travis-ci.org/garyttierney/rust-csp.svg?branch=master)](https://travis-ci.org/garyttierney/rust-csp)

[![COPR Status](https://copr.fedorainfracloud.org/coprs/gtierney/cspc/package/cspc/status_image/last_build.png)](https://copr.fedorainfracloud.org/coprs/gtierney/cspc/)

## Introduction

secsp is a C-style SELinux policy language for the High-Level Language Infrastructure.  It targets CIL, and takes inspiration from
refpolicy and the kernel policy language in it's syntax.

*NOTE: A majority of the parser and compiler is still largely unimplemented*

## What is implemented?

The core language constructs are currently implemented.  That is,
access vector rules, symbol declarations, if-else statements, blocks, macros and expressions.

However, there is a lot still to be implemented.  A partially complete list of those missing features can be found below:

* [ ] Type Enforcement
   * [ ] Transition rules
   * [x] Access vector
   * [ ] Extended access vector rules
   * [ ] Constrain statements
   * [x] Class permission expressions
* [ ] Labeling
   * [ ] Network object labeling
   * [ ] Filesystem object labeling
   * [ ] ISID labeling
* [ ] Attributes
   * [ ] Attribute modifiers
* [x] Macros
    * [x] Macro calls
    * [x] Macro declarations
* [ ] Declaration initializers
   * [x] Context
   * [x] Level
   * [x] Level Range
   * [ ] Category set
   * [ ] Attributes
   * [ ] Class permissions
   * [ ] Classes
* [x] Structural
   * [x] Block inheritance
   * [x] Abstract blocks
   * [x] If-else-if-else statements

## Installation

Experimental builds of `cspc` are currently available via Fedora's COPR build system.  To install the compiler, enable the following repository and install `cspc` with dinf.

```sh
> $ dnf copr enable gtierney/cspc
> $ dnf install cspc
```

## Building

secsp can be built by using the Cargo packaging utility.

```shell
> $ cargo build --release
```

## Usage

### Running `cspc` directly

```
USAGE:
    cspc [FLAGS] [OPTIONS]

FLAGS:
    -d, --decompile    Decompile CIL sources into equivalent CSP
    -s, --show_ast     Print the parsed AST to stdout
    -h, --help         Prints help information
    -V, --version      Prints version information

OPTIONS:
    -f, --file <INPUT>    Sets the input file to use
```

If no options are given, `cspc` will attempt to compile input from `stdin`.

### Using semodule

`semodule` allows policy compilers to be placed under `/usr/libexec/selinux/hll/` so the compiler can find them.  If using the built package from the copr repository above a symlink from `/usr/bin/cspc` to `/usr/libexec/selinux/hll/csp` will exist, allowing semodule to import policy files suffixed with `.csp`.

To import one of these files into the policy store run `semodule -i myfile.csp`.  *NOTE*: this isn't recommended at the moment given `cspc` is in an experimental state.