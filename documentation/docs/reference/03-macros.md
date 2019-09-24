---
id: reference--macros
title: Macros
sidebar_label: Macros
---

Macros provide a way to define reusable blocks of policy that can be
tuned by parameterizing them. When a macro is called, the call is
expanded to the macro body with any references to parameters substituted
out for their arguments.

For those familiar with the C preprocessor, the macros in CSP work in a
similar fashion. The distinguishing feature is that macros in CSP are
strongly typed and are preserved as symbols in the final compiled policy
database.

Examples
========

    macro allow_file_read(type source, type target) {
        allow source target : file (read);
    }

    type a;
    type b;

    allow_file_read(a, b);
