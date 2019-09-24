---
id: developer--testing
title: Testing
sidebar_label: Testing
---

Comprehensive testing of the language is a necessity to ensure stability
and robustness going forward. A major part of this is making the process
of adding new tests accessible. Each component of the compiler and it’s
associated libraries try to do this by providing an easy-to-use test
interface for the majority of test cases.

Parser tests
============

Writing tests for the parser itself can be done by using an XML-like DSL
on top of regular CSP policy code.

    <marker type="container">block a <marker type="block">{}</marker></marker>

These markers will be extracted from the test fixture and used to assert
that the AST nodes occurring at the starting and closing positions of
the tags match the `type` value of the marker.

Markers are not restricted to the top-level node, and can appear
anywhere within a source-file:


    block a {
        <marker type="macrocall">test();</marker>
    }

Existing examples of these tests can be found in the `libsecsp-syntax`
crate under the grammar module. An example test is given below:

```rust
    #[test]
    fn parse_block() {
        crate::grammar::test::test_parser(r#"
            block a <block type="block">{}</block>
        "#)
    }
```
    
