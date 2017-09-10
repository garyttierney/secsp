# Conditionals

Conditional statements look similar to C-style if-statements, with optional parenthesis.  In contrast to CIL, else-if branches are also supported in CIL.

Example:

```csp
if my_bool {
    // if...
} else if my_other_bool {
    // else if...
} else {
    // else
}
```

This works by creating new `false` branches for each else-if present in the statement.

Output:
```cil
(booleanif my_bool
    (true ; if...
    )
    (false
        (booleanif my_other_bool
            (true ; else if...
            )
            (false ; else...
            )
        )
    )
)
```