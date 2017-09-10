# Blocks

Blocks in CSP are very similar to those in CIL.  The one difference is that `block` namespaces allow an an optional `abstract` modifier, allowing shorthand
for a CIL `blockabstract` statement.

Example:

```csp
block a {

}

abstract block b {

}
```

Output:

```cil
(block a)
(block b
    (blockabstract b)
)
```