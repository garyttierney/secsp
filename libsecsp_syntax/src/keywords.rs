macro_rules! keyword {
    ($name:ident: $value:expr; $documentation:expr) => (
        #[doc=$documentation]
        pub const $name: &'static str = $value;
    );
}

keyword!(
    TYPE: "type";
    "The `type` statement keyword, which declares a new named security type."
);

keyword!(
    TYPE_ATTRIBUTE: "type_attribute";
    "The `type_attribute` statement keyword, which declares a new bitset of types."
);

keyword!(
    ROLE: "role";
    "The `role` statement keyword, which declares a new role for role-based access control."
);

keyword!(
    ROLE_ATTRIBUTE: "role_attribute";
    "The `role_attribute` statement keyword, which declares a new bitset of roles."
);

keyword!(
    USER: "user";
    "The `user` statement keyword, which declares a new security identity."
);

keyword!(
    USER_ATTRIBUTE: "user_attribute";
    "The `user_attribute` statement keyword, which declares a new bitset of users"
);

keyword!(
    OPTIONAL: "optional";
    "The `optional` statement keyword, which declares a new optional container"
);

keyword!(
    CONTEXT: "context";
    "The `context` statement keyword, which declares a new set of security attributes"
);

keyword!(
    SENSITIVITY: "sensitivity";
    "The `sensitivity` statement keyword, which declares a new multi-level security sensitivity level"
);

keyword!(
    CATEGORY: "category";
    "The `category` statement keyword, which declares a new multi-level security compartment for compartmentalization"
);

keyword!(
    LEVEL_RANGE: "level_range";
    "The `level_range` statement keyword, which declares a new low and high pair of sensitivities and category sets"
);

keyword!(
    BLOCK: "block";
    "The `block` statement keyword, which declares a new namespace container"
);

keyword!(
    IN: "in";
    "The `in` statement keyword, which extends an existing namespace container"
);

keyword!(
    ABSTRACT: "abstract";
    "The `abstract` modifier, used to mark a `block` as abstract"
);

keyword!(
    INHERITS_FROM: "inherits_from";
    "The `abstract` modifier, used to mark a `block` as abstract"
);

keyword!(
    ALLOW: "allow";
    "The `allow` statement keyword, which represents an allowed type-enforcement rule"
);

keyword!(
    AUDIT_ALLOW: "audit_allow";
    "The `audit_allow` statement keyword, which represents a type-enforcement rule that logs when allowed"
);

keyword!(
    NEVER_ALLOW: "never_allow";
    "The `never_allow` statement keyword, which represents a build time type-enforcement check on `allow` rules"
);

keyword!(
    DONT_AUDIT: "dont_audit";
    "The `dont_audit` statement keyword, which represents a type-enforcement rule that prevents logging when denied"
);

pub fn is_symbol_ty(val: &str) -> bool {
    match val {
        TYPE | TYPE_ATTRIBUTE | ROLE | ROLE_ATTRIBUTE | USER | USER_ATTRIBUTE => true,
        _ => false,
    }
}
