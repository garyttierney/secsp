---
id: user--setup
title: Setup
---

The following installation options are currently available for secsp:

- Source Build

## Source Build

The latest tree in master at https://github.com/garyttierney/secsp always contains a buildable branch that can be built with stable Rust.

```shell script
> $ git clone https://github.com/garyttierney/secsp.git
> $ cd secsp
> $ cargo build --release
> $ ./target/release/secsp {...}
```
