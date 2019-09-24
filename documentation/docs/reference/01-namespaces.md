---
id: reference--namespaces
title: Namespaces
sidebar_label: Namespaces
---

A namespace in CSP is a named container that can contain zero or more
*block items*, that is, a policy statement or declaration. Any block
items at the top-level are implicitly placed under the root namespace,
and can be explicitly accessed from another any other scope by prefixing
the referenced path with a leading period: `.root_item`.

Though the most common type of namespace is a simple container declared
with the `block` keyword, there are three types of namespace:

Optional  
A namespace that will be omitted from the generated policy database if
any of its code is semantically invalid. For example: unresolved
references to policy symbols or macros.

> **Note**
>
> This does not exempt the block from parse errors, which will still
> cause compilation of the entire source file to fail.

Extension  
Extension namespaces work like a regular `block` namespace, but adds and
inherits items to/from a pre-defined namespace. The extension is created
with the `in` keyword and has access to all block items declared within
the namespace it extends, and all other extensions of the same
namespace.

Block  
Declares a new namespace name, with the option of being an *abstract*
namespace. Any other namespace can inherit from any number of *abstract*
block namespaces, the result being that the block items of the abstract
namespace become available in the child.

Examples:

    abstract block parent {
        type src;

        macro test_read(type t) {
            if allow_parent_test_read {
                allow src target : file (read);
            }
        }
    }

    block child extends parent {
        type tgt;
        test_read(tgt);
    }

    optional test {
        type t;
        allow t unresolved : file (read); // reference to unresolved will cause this block to be skipped
    }

    block test {
        type_attribute types;
    }

    in test {
        type t;
        types |= t;
    }
