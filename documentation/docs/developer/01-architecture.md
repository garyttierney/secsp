---
id: developer--architecture
title: Architecture
sidebar_label: Architecture
---

The implementation of the `secsp` language is split across several
re-usable Rust crates that are consumed by both the compiler and
analysis server. The intention is that the compiler and analysis server
rely on the same code to do their job and can be developed in parallel.

More details on the individual crates can be found below.

secsp-syntax
============

The parser implementation and AST representation.

This crate is inspired by the work done by JetBrains on their [PSI
architecture](https://www.jetbrains.org/intellij/sdk/docs/basics/architectural_overview/psi.html)
and the work done on the
[rust-analyzer](https://github.com/rust-analyzer/rust-analyzer) project.
