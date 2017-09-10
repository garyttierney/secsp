# secsp

[![Coverage Status](https://coveralls.io/repos/github/garyttierney/rust-csp/badge.svg?branch=master)](https://coveralls.io/github/garyttierney/rust-csp?branch=master)

[![Build Status](https://travis-ci.org/garyttierney/rust-csp.svg?branch=master)](https://travis-ci.org/garyttierney/rust-csp)

[![COPR Status](https://copr.fedorainfracloud.org/coprs/gtierney/cspc/package/cspc/status_image/last_build.png)](https://copr.fedorainfracloud.org/coprs/gtierney/cspc/)

## Introduction

secsp is a C-style SELinux policy language for the High-Level Language Infrastructure.  It targets CIL, and takes inspiration from
refpolicy and the kernel policy language in it's syntax.

*NOTE: A majority of the parser and compiler is still largely unimplemented*

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


