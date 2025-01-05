use il2cpp::vm::{Il2cppClass, Il2cppMethod, Il2cppString};

pub struct MetadataEntry {
    pub address: usize,
    pub usage: MetadataUsage,
}

pub enum MetadataUsage {
    TypeInfo(Il2cppClass),
    MethodRef(Il2cppMethod),
    StringLiteral(Il2cppString),
}

pub const USAGES_COUNT: usize = 0x38845;

const USAGE_TYPE_INFO: u32 = 1;
const USAGE_IL2CPP_TYPE: u32 = 2;
const USAGE_METHOD_DEF: u32 = 3;
const USAGE_FIELD_INFO: u32 = 4;
const USAGE_STRING_LITERAL: u32 = 5;
const USAGE_METHOD_REF: u32 = 6;

pub fn get_usage_by_index(index: usize) -> Option<MetadataEntry> {
    const GLOBAL_METADATA: usize = 0xB225F30;
    const TYPE_INFO_TABLE: usize = 0xB2277F0;
    const METHOD_REF_TABLE: usize = 0xB2A9C90;
    const STRING_LITERAL_TABLE: usize = 0xB3A1890;

    assert!(
        index < USAGES_COUNT,
        "usage index out of range: {index}/{USAGES_COUNT}"
    );

    unsafe {
        let global_metadata = *((il2cpp::ffi::base() + GLOBAL_METADATA) as *const usize);
        let v10 = (*((global_metadata + 41952208 + 8 * index + 4) as *const i32) ^ 0x56089010)
            - 1344935350 * ((-849621129 * (index as i32) - 214430094) ^ 0x3939D59A);
        let v11 = (*((global_metadata + 41952208 + 8 * index) as *const i32) ^ 0x6EFCC6A2)
            - 1344935350 * ((-849621129 * (index as i32) - 214430094) ^ 0x3939D59A);

        Some(match v10 as u32 >> 29 {
            USAGE_TYPE_INFO => {
                let address = il2cpp::ffi::base() + TYPE_INFO_TABLE + ((v11 as usize) * 8);
                MetadataEntry {
                    address,
                    usage: MetadataUsage::TypeInfo(Il2cppClass(dereference(address))),
                }
            }
            USAGE_IL2CPP_TYPE => panic!("USAGE_IL2CPP_TYPE is not supported in this build"),
            USAGE_METHOD_REF | USAGE_METHOD_DEF => {
                let address = il2cpp::ffi::base() + METHOD_REF_TABLE + ((v11 as usize) * 8);
                MetadataEntry {
                    address,
                    usage: MetadataUsage::MethodRef(Il2cppMethod(dereference(address))),
                }
            }
            USAGE_FIELD_INFO => return None,
            USAGE_STRING_LITERAL => {
                let address = il2cpp::ffi::base() + STRING_LITERAL_TABLE + ((v11 as usize) * 8);
                MetadataEntry {
                    address,
                    usage: MetadataUsage::StringLiteral(Il2cppString(dereference(address))),
                }
            }
            other => todo!("metadata usage type {other} is not implemented"),
        })
    }
}

unsafe fn dereference(address: usize) -> *const u8 {
    *(address as *const usize) as *const u8
}
