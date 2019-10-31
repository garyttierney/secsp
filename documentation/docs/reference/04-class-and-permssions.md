---
id: reference--class-and-permissions
title: Classes and Permissions
sidebar_label: Classes and Permissions
---

## Class Definitions
 
A security class defines a type of SELinux [object](TODO-glossary), or in some cases, [subject](TODO-glossary), and the operations that can be performed on it.

There are 3 variants of security classes that occur in secsp policy.

### Common Class 

A type of abstract security class that models a set of base permissions, to be extended by other concrete security classes. Exists to build a framework for other security classes, and cannot be referenced in class permission expressions.

### Class

A security class that directly maps to an object class defined in the Linux kernel or by a [userspace object manager](TODO-glossary).

### Class Map

A type of security class containing permissions that may be an aggregate of 0 or more other security classes.

These variants can be combined and provide the building blocks for modeling access vectors in the policy language.

### Example 

```csp
common file {
    ioctl
    read
    write
    create
    getattr
    setattr
    lock
    relabelfrom
    relabelto
    append
    map
    unlink
    link
    rename
    execute
    swapon
    quotaon
    mounton
    audit_access
    open
    execmod
    watch
    watch_mount
    watch_sb
    watch_with_perm
    watch_reads
}

class dir inherits file {
    add_name
    remove_name
    reparent
    search
    rmdir
}

class file inherits file {
    execute_no_trans
    entrypoint
}

class lnk_file inherits file;

class chr_file inherits file {
    execute_no_trans
    entrypoint
}

class_map all_files {
    read
    write
}

class_mapping all_files read : { dir file lnk_file chr_file } read;
class_mapping all_files write : { dir file lnk_file chr_file } write; 
```

## Class Permission Sets

A class permission set is an identifier that associates a security class and one or more permissions to form a named set.
Nested expressions may be used to determine the selected permissions that form the set.

### Lists

The usual way to define a class permission set and is using a class permission list expression.
These expressions reference a security class by name and 1 or more permissions.
The operator `*` may be used to refer to all permissions.

```csp
class_permission example = security_class { permission };
```

If only one permission is present the braces may be omitted:

```csp
class_permission example = security_class permission;
```

As well as associating permissions with the set, you can also negate the entire set, or remove individual items:

```csp
class_permission example = security_class ~{ perm1 perm2 };
class_permission example2 = security_class { * -perm2 };
```

### Expressions

In addition to lists, a more flexible syntax is available that allows the policy author to compose new permission definitions out of existing sets.
These are represented as binary expressions using the operators `&` (and), `^` (xor), and `|` (or).

```csp
class_permission example = security_class permission_set_1 & ~permission_set_2;
class_permission example2 = security_class ~{ perm1 } & permission_set_1;
```

### Example

```csp
class binder {
    impersonate call
    set_context_mgr
    transfer receive
}

class property_service { set }

class zygote  {
    specifyids
    specifyrlimits
    specifycapabilities
    specifyinvokewith
    specifyseinfo
}

class_map android_classes { 
    set1
    set2
    set3
}

class_mapping android_classes set1 : binder *;
class_mapping android_classes set1 : property_service set;
class_mapping android_classes set1 : zygote ~specifyids;

class_mapping android_classes set2 : binder { impersonate call set_context_mgr transfer };
class_mapping android_classes set2 : zygote { specifyids specifyrlimits specifycapabilities specifyinvokewith };

class_permission cps_zygote = zygote ~specifyids;
class_mapping android_classes set3 : cps_zygote;
class_mapping android_classes set3 : binder { * -transfer };
 
class_permission read = all_files ~write;
class_permission read_write = all_files { read write };
```
