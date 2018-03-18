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
    OPTIONAL: "optional";
    "The `optional` statement keyword, which declares a new optional container"
);

keyword!(
    BLOCK: "block";
    "The `block` statement keyword, which declares a new namespace container"
);

keyword!(
    ABSTRACT: "abstract";
    "The `abstract` modifier, used to mark a `block` as abstract"
);
