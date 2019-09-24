---
id: reference--conditionals
title: Conditionals
sidebar_label: Conditionals
---

Conditional statements allow policy authors to disable sections of
policy dependent on the value of toggles that can be set at either
runtime or build time. A common case for utilizing conditionals is to
write policy for the common case and provide toggles that can be tuned
for edge cases.

The syntax is similar to C, however is relaxed in that parenthesis do
not need to wrap the predicate expression of the conditional statement.

Examples
========

    if allow_file_manage && other_bool {
        allow src file_type : manage;
    } else if allow_file_write {
        allow src file_type : write;
    } else {
        allow src file_type : read;
    }
