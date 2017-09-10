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
* [ ] Structural
   * [ ] Block inheritance
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

## Running

The compiler can be ran by invoking the built executable and passing a CSP source file in through `stdin`.

```shell
> $ target/release/csp < examples/block_with_macros.csp
```