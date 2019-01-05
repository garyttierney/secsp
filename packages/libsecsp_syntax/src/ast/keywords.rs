use crate::ast::SyntaxKind;
use std::str::FromStr;

macro_rules! define_keywords {
    ($($name:ident: $value:expr; $documentation:expr),*) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        pub enum Keyword {
            $(
                #[doc=$documentation]
                $name,
            )*
        }

        impl Keyword {
            pub fn from_str<S: AsRef<str>>(value: S) -> Option<Self> {
                match value.as_ref() {
                    $($value => Some(Keyword::$name),)*
                    _ => None
                }
            }
        }

        $(
            #[doc=$documentation]
            pub const $name: &'static str = $value;
        )*
    }
}

impl Into<SyntaxKind> for Keyword {
    fn into(self) -> SyntaxKind {
        SyntaxKind::Keyword(self)
    }
}

impl Keyword {
    pub fn is_var_type(&self) -> bool {
        match self {
            Keyword::TYPE
            | Keyword::TYPE_ATTRIBUTE
            | Keyword::ROLE
            | Keyword::ROLE_ATTRIBUTE
            | Keyword::USER
            | Keyword::USER_ATTRIBUTE
            | Keyword::SENSITIVITY
            | Keyword::CATEGORY
            | Keyword::LEVEL_RANGE => true,
            _ => false,
        }
    }
}

define_keywords!(
    TYPE: "type";
    "The `type` statement keyword, which declares a new named security type.",

    TYPE_ATTRIBUTE: "type_attribute";
    "The `type_attribute` statement keyword, which declares a new bitset of types.",

    ROLE: "role";
    "The `role` statement keyword, which declares a new role for role-based access control.",

    ROLE_ATTRIBUTE: "role_attribute";
    "The `role_attribute` statement keyword, which declares a new bitset of roles.",

    USER: "user";
    "The `user` statement keyword, which declares a new security identity.",

    USER_ATTRIBUTE: "user_attribute";
    "The `user_attribute` statement keyword, which declares a new bitset of users",

    OPTIONAL: "optional";
    "The `optional` statement keyword, which declares a new optional container",

    SENSITIVITY: "sensitivity";
    "The `sensitivity` statement keyword, which declares a new multi-level security sensitivity level",

    CATEGORY: "category";
    "The `category` statement keyword, which declares a new multi-level security compartment for compartmentalization",

    LEVEL_RANGE: "level_range";
    "The `level_range` statement keyword, which declares a new low and high pair of sensitivities and category sets",

    BLOCK: "block";
    "The `block` statement keyword, which declares a new namespace container",

    IN: "in";
    "The `in` statement keyword, which extends an existing namespace container",

    ABSTRACT: "abstract";
    "The `abstract` modifier, used to mark a `block` as abstract",

    EXTENDS: "extends";
    "The `extends` keyword, used to begin an inheritance list",

    ALLOW: "allow";
    "The `allow` statement keyword, which represents an allowed type-enforcement rule",

    AUDIT_ALLOW: "audit_allow";
    "The `audit_allow` statement keyword, which represents a type-enforcement rule that logs when allowed",

    NEVER_ALLOW: "never_allow";
    "The `never_allow` statement keyword, which represents a build time type-enforcement check on `allow` rules",

    DONT_AUDIT: "dont_audit";
    "The `dont_audit` statement keyword, which represents a type-enforcement rule that prevents logging when denied",

    MACRO: "macro";
    "The `macro` statement keyword, which declares a new named macro in the current namespace"
);
