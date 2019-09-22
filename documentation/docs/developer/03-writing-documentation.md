---
id: developer--writing-documentation
title: Writing Documentation
sidebar_label: Writing Documentation
---

Since this project is still in its infancy, a lot of the documentation
is missing important details or absent entirely, including this document
itself. This is intended to serve as a guide for adding new
documentation going forward and may be refined as seen fit.

Writing new reference documentation
===================================

The reference documentation is intended to provide a detailed overview
of the language syntax and semantics, as well as how to use them.

When adding new reference documentation sections it is important to
explain the feature being documented within the confines of the language
itself, rather than tying it to the equivalent SELinux kernel policy
object and relying on the users understanding of that.

As an example, prefer the long-form description of what an `sid` is as
opposed to describing it as an "SELinux initial security identifier."
This helps explains the goals of various policy constructs and makes the
documentation more accessible to those who aren’t SELinux policy
experts.
