use std::collections::BTreeMap;

use il2cpp::vm::Il2cppDomain;

pub struct TypeCache {
    pub type_map: BTreeMap<usize, CachedType>,
}

#[derive(PartialEq)]
pub enum CachedType {
    Object,
    Boolean,
    Byte,
    SByte,
    UInt16,
    Int16,
    UInt32,
    Int32,
    UInt64,
    Int64,
    Single,
    Double,
    String,
    ByteString,
    Any,
    Enum,
}

impl TypeCache {
    pub fn init(domain: &Il2cppDomain) -> Self {
        use CachedType::*;
        let corlib = domain.assembly_open("mscorlib.dll").image();
        let protobuf_lib = domain.assembly_open("Protobuf.dll").image();

        TypeCache {
            type_map: BTreeMap::from([
                (
                    corlib.get_class_by_name("System", "Object").unwrap().0 as usize,
                    Object,
                ),
                (
                    corlib.get_class_by_name("System", "Boolean").unwrap().0 as usize,
                    Boolean,
                ),
                (
                    corlib.get_class_by_name("System", "Byte").unwrap().0 as usize,
                    Byte,
                ),
                (
                    corlib.get_class_by_name("System", "SByte").unwrap().0 as usize,
                    SByte,
                ),
                (
                    corlib.get_class_by_name("System", "UInt16").unwrap().0 as usize,
                    UInt16,
                ),
                (
                    corlib.get_class_by_name("System", "Int16").unwrap().0 as usize,
                    Int16,
                ),
                (
                    corlib.get_class_by_name("System", "UInt32").unwrap().0 as usize,
                    UInt32,
                ),
                (
                    corlib.get_class_by_name("System", "Int32").unwrap().0 as usize,
                    Int32,
                ),
                (
                    corlib.get_class_by_name("System", "UInt64").unwrap().0 as usize,
                    UInt64,
                ),
                (
                    corlib.get_class_by_name("System", "Int64").unwrap().0 as usize,
                    Int64,
                ),
                (
                    corlib.get_class_by_name("System", "Single").unwrap().0 as usize,
                    Single,
                ),
                (
                    corlib.get_class_by_name("System", "Double").unwrap().0 as usize,
                    Double,
                ),
                (
                    corlib.get_class_by_name("System", "String").unwrap().0 as usize,
                    String,
                ),
                (
                    corlib.get_class_by_name("System", "Enum").unwrap().0 as usize,
                    Enum,
                ),
                (
                    protobuf_lib
                        .get_class_by_name("", super::BYTE_STRING)
                        .unwrap()
                        .0 as usize,
                    ByteString,
                ),
                (
                    protobuf_lib
                        .get_class_by_name("", super::PROTOBUF_ANY)
                        .unwrap()
                        .0 as usize,
                    Any,
                ),
            ]),
        }
    }
}
