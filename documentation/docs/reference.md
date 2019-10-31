---
id: reference
title: Syntax Reference
sidebar_label: Index
---

> **Note**
>
> While CSP is largely inspired by and ultimately compiles down to CIL
> policy, an understanding of CIL should not be required to understand
> the language reference documentation.

## Syntax

### Identifiers
  
An identifier is any valid ASCII string that begins with an alpha
character or `\_`, and follows with any alphanumeric character or `_`.
An example regular expression matching this pattern wouuld be:
`[a-zA-Z_][a-zA-Z0-9_]*`.

## Policy Structure

The fundamental building blocks of policy structure that differentiate
CSP from the simple kernel policy language.

### [Namespaces](#01-namespaces.adoc#)  
Namespaces provide a structured approach to building blocks of reusable
policy.

### [Conditionals](#02-conditionals.adoc#)  
Conditionals allow the policy author to disable/enable policies based on
the value of runtime or build-time toggles.

### [Macros](#03-macros.adoc#)  
Function-like blocks of policy that can be parameterized and invoked.
