use il2cpp::{
    ffi::{GLOBAL_METADATA, GLOBAL_METADATA_HEADER, method_info_from_index, string_literal_from_index}, vm::{Il2cppClass, Il2cppMethod, Il2cppString},
};

pub struct MetadataEntry {
    pub address: usize,
    pub usage: MetadataUsage,
}

pub enum MetadataUsage {
    TypeInfo(Il2cppClass),
    MethodRef(Il2cppMethod),
    StringLiteral(Il2cppString),
}

pub const USAGES_COUNT: usize = 993674;

const USAGE_TYPE_INFO: u32 = 1;
const USAGE_IL2CPP_TYPE: u32 = 7;
const USAGE_METHOD_DEF: u32 = 3;
const USAGE_FIELD_INFO: u32 = 4;
const USAGE_STRING_LITERAL: u32 = 5;
const USAGE_METHOD_REF: u32 = 6;

const METADATA_REGISTER_TABLE: usize = 0x4DA2C60;
pub fn get_usage_by_index(index: usize) -> Option<MetadataEntry> {

    assert!(
        index < USAGES_COUNT,
        "usage index out of range: {index}/{USAGES_COUNT}"
    );

    unsafe {
        let global_metadata_header =
            *((il2cpp::ffi::base() + GLOBAL_METADATA_HEADER) as *const usize);
        let global_metadata = *((il2cpp::ffi::base() + GLOBAL_METADATA) as *const usize);

        let metadata_usage_pairs_offset =
            *(global_metadata_header.wrapping_add(180) as *const u32) as usize;
        let metadata_unknown_offset =
            *(global_metadata_header.wrapping_add(164) as *const u32) as usize;
        let metadata_unknown2_offset =
            *(global_metadata_header.wrapping_add(368) as *const u32) as usize;

        let v6 = index as u64;
        let v9 = global_metadata + metadata_usage_pairs_offset - 660889280;
        let v11 = (1083960151u64 * ((2024793878u64 * ((31039 * v6) ^ 0x116BE0CF)) >> 18)
            + 0x1D48E5A1A9F5)
            >> 13;
        let v12 = (v11 as u32) ^ (*((v9 + 8 * index + 4) as *const u32)) - 1236877375;
        let v13 = (*((v9 + 8 * index) as *const u32)) ^ (v11 as u32) ^ 0x2A20EFCF;

        Some(match v12 as u32 >> 29 {
            USAGE_TYPE_INFO => {
                let reg = *((il2cpp::ffi::base() + METADATA_REGISTER_TABLE + 48) as *const usize);
                let address = reg + 8 * (v13 as usize);
                MetadataEntry {
                    address,
                    usage: MetadataUsage::TypeInfo(Il2cppClass(dereference(address))),
                }
            }
            USAGE_IL2CPP_TYPE | USAGE_FIELD_INFO => return None,
            USAGE_METHOD_REF | USAGE_METHOD_DEF => {
                let address = method_info_from_index(v12) as usize;
                MetadataEntry {
                    address: address,
                    usage: MetadataUsage::MethodRef(Il2cppMethod(address as *const u8)),
                }
            }
            USAGE_STRING_LITERAL => {
                let v14 = (v12 & 0x1FFFFFFF) as u64;
                let v19 = global_metadata + metadata_unknown_offset as i32 as usize - 916608662;
                let v20 = (*((v19 + 4 * v14 as usize) as *const u32))
                    ^ (432008534u64 * ((1251024346u64 * ((59008 * v14) ^ 0x69401238)) >> 10)
                        + 1022566152) as u32
                    ^ 0x50B11DC5;
                let param1 = v20 as i32 as i64
                    + global_metadata as i64
                    + metadata_unknown2_offset as i32 as i64
                    - 2053480590;
                let param2 = ((*((v19 + 4 * v14 as usize + 4) as *const u32))
                    ^ (432008534
                        * ((1251024346u64 * ((59008 * v14 + 59008) ^ 0x69401238)) >> 10) as u32
                        + 1022566152)
                    ^ 0x50B11DC5)
                    - v20;
                let address =
                    string_literal_from_index(v14 as u32, param1 as *const u8, param2 as *const u8)
                        as usize;
                MetadataEntry {
                    address,
                    usage: MetadataUsage::StringLiteral(Il2cppString(address as *const u8)),
                }
            }
            other => todo!("metadata usage type {other} is not implemented"),
        })
    }
}

unsafe fn dereference(address: usize) -> *const u8 {
    *(address as *const usize) as *const u8
}
