macro_rules! define_block {
    ($($name:ident $value:expr;)*) => {
        $(
            pub const $name: u32 = $value;
        )*
    };
}

// Field attribues
define_block! {
    FIELD_ATTRIBUTE_FIELD_ACCESS_MASK     0x0007;
    FIELD_ATTRIBUTE_COMPILER_CONTROLLED   0x0000;
    FIELD_ATTRIBUTE_PRIVATE               0x0001;
    FIELD_ATTRIBUTE_FAM_AND_ASSEM         0x0002;
    FIELD_ATTRIBUTE_ASSEMBLY              0x0003;
    FIELD_ATTRIBUTE_FAMILY                0x0004;
    FIELD_ATTRIBUTE_FAM_OR_ASSEM          0x0005;
    FIELD_ATTRIBUTE_PUBLIC                0x0006;

    FIELD_ATTRIBUTE_STATIC                0x0010;
    FIELD_ATTRIBUTE_INIT_ONLY             0x0020;
    FIELD_ATTRIBUTE_LITERAL               0x0040;
    FIELD_ATTRIBUTE_NOT_SERIALIZED        0x0080;
    FIELD_ATTRIBUTE_SPECIAL_NAME          0x0200;
    FIELD_ATTRIBUTE_PINVOKE_IMPL          0x2000;
}

// Method attributes
define_block! {
    METHOD_ATTRIBUTE_MEMBER_ACCESS_MASK        0x0007;
    METHOD_ATTRIBUTE_COMPILER_CONTROLLED       0x0000;
    METHOD_ATTRIBUTE_PRIVATE                   0x0001;
    METHOD_ATTRIBUTE_FAM_AND_ASSEM             0x0002;
    METHOD_ATTRIBUTE_ASSEM                     0x0003;
    METHOD_ATTRIBUTE_FAMILY                    0x0004;
    METHOD_ATTRIBUTE_FAM_OR_ASSEM              0x0005;
    METHOD_ATTRIBUTE_PUBLIC                    0x0006;

    METHOD_ATTRIBUTE_STATIC                    0x0010;
    METHOD_ATTRIBUTE_FINAL                     0x0020;
    METHOD_ATTRIBUTE_VIRTUAL                   0x0040;
    METHOD_ATTRIBUTE_HIDE_BY_SIG               0x0080;

    METHOD_ATTRIBUTE_VTABLE_LAYOUT_MASK        0x0100;
    METHOD_ATTRIBUTE_REUSE_SLOT                0x0000;
    METHOD_ATTRIBUTE_NEW_SLOT                  0x0100;

    METHOD_ATTRIBUTE_STRICT                    0x0200;
    METHOD_ATTRIBUTE_ABSTRACT                  0x0400;
    METHOD_ATTRIBUTE_SPECIAL_NAME              0x0800;

    METHOD_ATTRIBUTE_PINVOKE_IMPL              0x2000;
    METHOD_ATTRIBUTE_UNMANAGED_EXPORT          0x0008;
}
