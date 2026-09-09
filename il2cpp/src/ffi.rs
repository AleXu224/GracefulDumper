pub use crate::util::base;
use crate::util::{self, import};

pub const GLOBAL_METADATA_HEADER: usize = 0x4C24FA0;
pub const GLOBAL_METADATA: usize = 0x4C24FA8;

import!(il2cpp_gc_disable() -> () = 0x93A920); // 74 ? ? ? ? ? ? ? ? FF 15 ? ? ? ? ? ? ? ? ? ? ? FF 05 ? ? ? ? ? ? 75
import!(il2cpp_domain_get() -> usize = 0x93A410);
import!(il2cpp_domain_assembly_open(domain: usize, name: *const u8) -> usize = 0x93A440); // ".dll" search
import!(il2cpp_domain_get_assemblies(domain: usize, size: &mut usize) -> *const usize = 0x93A450); // 48 8B 05 ? ? ? ? 48 29 C1
import!(il2cpp_assembly_get_image(assembly: *const u8) -> *const u8 = 0x939F70); // In Runtime::Init right at "mscorlib.dll"
import!(il2cpp_class_from_name(image: *const u8, namespace: *const u8, name: *const u8) -> *const u8 = 0x93A000); // function that jumps to 48 C7 45 ? ? ? ? ? 48 89 55 ? ? ? ? ? ? ? ? ? ? ? ? ? ? ? ? ? ? 4C 89 45
import!(il2cpp_image_get_class_count(image: *const u8) -> usize = 0x93B720); // Can find the XOR constant inside Image::ClassFromName from il2cpp_class_from_name
import!(il2cpp_image_get_class(image: *const u8, index: usize) -> *const u8 = 0x93B730); // Just under il2cpp_image_get_class_count
import!(il2cpp_image_get_name(image: *const u8) -> *const i8 = 0x93B710); // Just above il2cpp_image_get_class_count
import!(il2cpp_class_from_il2cpp_type(il2cpp_type: *const u8) -> *const u8 = 0x277B80); // Find "GenericClass" in Runtime::Init and get the global that it initializes, if you xref that then the next function is gonna be this
import!(il2cpp_field_get_name(field: *const u8) -> *const i8 = 0x93A650); // Find Enum::GetEnumValuesAndNames next to "value__"
import!(il2cpp_field_get_type(field: *const u8) -> *const u128 = 0x93A6A0); // get type XOR from InitLocked > SetupFieldsLocked > SetupFieldsFromDefinitionLocked
import!(il2cpp_field_get_offset(field: *const u8) -> u32 = 0x93A690); // Right between the previous two functions
import!(il2cpp_vm_class_init(class: *const u8) -> () = 0x26BC40); // 48 C7 45 ? ? ? ? ? F6 81
import!(il2cpp_vm_class_init_methods(class: *const u8) -> () = 0x280B40); // 48 C7 45 ? ? ? ? ? 31 C0 48 85 C9
import!(il2cpp_type_get_name(il2cpp_type: *const u128) -> *const i8 = 0x93B430); // 48 C7 45 ? ? ? ? ? 48 89 CA 0F 57 C0 0F 29 45 ? 48 C7 45 ? ? ? ? ? 48 C7 45 ? ? ? ? ? 48 8D 7D ? 48 89 F9 45 31 C0
import!(il2cpp_field_static_get_value(field: *const u8, out: *const usize) -> () = 0x2853F0); // Find xref of SetupFieldsLocked
import!(il2cpp_method_get_name(method: *const u8) -> *const i8 = 0x290720); // Find FormatExceptionMessageForNonGenericMethod using "is not a generic method" and get the xor that can be found right after the "::" lines
import!(il2cpp_class_get_method_from_name(class: *const u8, name: *const i8, args_count: i32) -> *const u8 = 0x93A0F0); // CreateUnhandledExceptionEventArgs > Go to the supposed Class::GetMethodFromNameFlags > xref the function between the mutex > the first function should be what you are looking for > xref that function and find the usage with two zeros for the last two params
import!(il2cpp_method_get_return_type(method: *const u8) -> *const u8 = 0x290830); // 48 B9 ? ? ? ? ? ? ? ? ? ? ? 48 B9 ? ? ? ? ? ? ? ? 48 89 48 ? 48 B9 ? ? ? ? ? ? ? ? 48 89 48 ? 48 89 46
import!(il2cpp_array_new(ty: *const u8, size: u64) -> *const u8 = 0x26BAE0); // Second xref on InitLocked
import!(il2cpp_object_new(class: *const u8) -> *const u8 = 0x292330); // 48 C7 45 ? ? ? ? ? 48 89 CE F6 81 ? ? ? ? ? 75 ? 48 8D 05 ? ? ? ? 48 89 45 ? 48 8B 0D ? ? ? ? FF 15 ? ? ? ? 48 8D 55 ? 48 89 F1 E8 ? ? ? ? 48 8B 45 ? ? ? ? FF 15 ? ? ? ? 48 8B 46
import!(il2cpp_runtime_invoke(method: *const u8, obj: *const u8, params: *const usize, exception: &mut usize) -> usize = 0x294F10); // 48 83 EC ? 48 8D 6C 24 ? 48 C7 45 ? ? ? ? ? 4C 89 C6 48 89 D3 48 89 CF 4D 85 C9
import!(il2cpp_thread_attach(domain: *const u8) -> () = 0x29DD80); // 48 C7 45 ? ? ? ? ? 48 89 CE 48 8B 05 ? ? ? ? ? ? FF 15

import!(metadatacache_getstringfromindex(param: u32) -> *const i8 = 0x279270); // Inside FromTypeDefinition from InitLocked

// Had no implementation in the binary
pub unsafe fn il2cpp_field_get_token(field: *const u8) -> u32 {
    // Found in SetupFieldsLocked
    *(field.wrapping_add(24) as *const u32) ^ 0x1818791C
}

pub unsafe fn il2cpp_class_get_name(class: *const u8) -> *const i8 {
    // Inside InitLocked when creating the error message
    *(class.wrapping_add(0x28) as *const usize) as *const i8
}

pub unsafe fn il2cpp_is_fully_initialized() -> bool {
    const S_INITIALIZEDIL2CPPFROMWINDOWSRUNTIME: usize = 0x4C24F78;

    *(util::base().wrapping_add(S_INITIALIZEDIL2CPPFROMWINDOWSRUNTIME) as *const u8) != 0
}

pub unsafe fn il2cpp_ptr_base() -> u64 {
    // A 48 Byte struct heap allocated struct that is initialized in Runtime::Init() that holds an arary that is used everywhere
    // The first field seems to be
    const STRUCT_OFFSET: usize = 0x4C24B50;

    let struct_ptr = util::base().wrapping_add(STRUCT_OFFSET) as *const u64;
    let first_field_ptr = *struct_ptr.wrapping_add(0) as *const u64;

    *first_field_ptr
}

pub unsafe fn il2cpp_class_get_token(class: *const u8) -> u32 {
    // InitLocked > SetupInterfacesLocked > fromIl2CppType > GenericClass::GetClass >
    *(class.wrapping_add(0xA4) as *const u32)
}

pub unsafe fn il2cpp_method_get_arg_count(method: *const u8) -> usize {
    // Inside Class::GetMethodFromNameFlags
    *method.wrapping_add(50) as usize // Stable 3.0 - 3.2 possibly not changing?
}

pub unsafe fn il2cpp_method_get_attrs(method: *const u8) -> u32 {
    *method.wrapping_add(0x2C) as u32
}

pub unsafe fn il2cpp_method_get_class(method: *const u8) -> *const u8 {
    *(method.wrapping_add(8) as *const usize) as *const u8
}

import!(il2cpp_vm_method_get_arguments(method: *const u8) -> *const u8 = 0x290A90); // Found inside PlatformInvoke::MarshalDelegate
pub unsafe fn il2cpp_method_get_arg_name(method: *const u8, index: usize) -> *const i8 {
    let args = il2cpp_vm_method_get_arguments(method);
    *(args.wrapping_add(0 + 24 * index) as *const usize) as *const i8
}

pub unsafe fn il2cpp_method_get_arg_type(method: *const u8, index: usize) -> *const u128 {
    let args = il2cpp_vm_method_get_arguments(method);
    *(args.wrapping_add(8 + 24 * index) as *const usize) as *const u128
}

pub unsafe fn il2cpp_class_get_methods(class: *const u8, count: &mut usize) -> *const usize {
    *count = *class.wrapping_add(0xC2).cast::<u16>() as usize;
    (*class.wrapping_add(0x68).cast::<usize>()) as *const usize
}

pub unsafe fn il2cpp_method_get_address(method: *const u8) -> usize {
    *method.wrapping_add(0).cast::<usize>()
}

pub unsafe fn il2cpp_type_get_attrs(ty: *const u128) -> u32 {
    *(ty.wrapping_byte_add(8) as *const u32)
}

pub unsafe fn il2cpp_class_get_interfaces(class: *const u8, count: &mut usize) -> *const usize {
    // SetupInterfacesLocked
    let type_definition = (class.wrapping_add(0x58) as *const usize).read() as *const u8;
    *count = *(type_definition.wrapping_add(0x46) as *const u16) as usize;
    *(class.wrapping_add(64) as *const usize) as *const usize
}

pub unsafe fn il2cpp_class_get_type_defition(class: *const u8) -> *const u8 {
    // SetupInterfacesLocked
    *(class.wrapping_add(88) as *const usize) as *const u8
}

pub unsafe fn il2cpp_type_definition_get_namespace_index(type_definition: *const u8) -> u32 {
    // InitLocked inside the error string creation
    *(type_definition.wrapping_add(32) as *const u32)
}

pub unsafe fn il2cpp_type_definition_get_field_count(type_definition: *const u8) -> u16 {
    *(type_definition.wrapping_add(0x3E) as *const u16)
}

pub unsafe fn il2cpp_class_get_type(class: *const u8) -> *const u128 {
    class.wrapping_add(128).cast::<u128>() // in-place struct
}

pub unsafe fn il2cpp_class_get_image(class: *const u8) -> *const u8 {
    *(class.wrapping_add(0) as *const usize) as *const u8
}

pub unsafe fn il2cpp_class_get_fields(class: *const u8) -> *const u8 {
    *(class.wrapping_add(0x38) as *const usize) as *const u8
}

pub unsafe fn il2cpp_class_get_generic_class(class: *const u8) -> *const u8 {
    (*class.wrapping_add(0x60).cast::<usize>()) as *const u8
}

pub unsafe fn il2cpp_class_get_parent(class: *const u8) -> *const u8 {
    let parent_offset = *class.wrapping_add(160).cast::<u32>();
    if parent_offset == 0 {
        return std::ptr::null();
    }
    let parent_array = il2cpp_ptr_base();
    (parent_offset as u64 + parent_array) as *const u8
}

pub unsafe fn il2cpp_generic_class_get_generic_container(
    // Class::IsAssignableFrom, find it by finding a function where InitLocked is used twice, right between InitLocked and runtime init
    generic_class: *const u8,
    index: &mut i64,
) -> *const u8 {
    let type_definition_index = *generic_class.wrapping_add(0).cast::<u32>();
    let global_metadata_header = *((base() + GLOBAL_METADATA_HEADER) as *const usize);
    let global_metadata = *((base() + GLOBAL_METADATA) as *const usize);

    let metadata_type_definitions_offset =
        *(global_metadata_header.wrapping_add(228) as *const u32);
    let metadata_generic_containers_offset =
        *(global_metadata_header.wrapping_add(92) as *const u32);

    let type_definition = global_metadata
        .wrapping_add(metadata_type_definitions_offset.wrapping_sub(1582360463) as usize)
        .wrapping_add(80usize * (type_definition_index as usize));
    let generic_container_index = *(type_definition.wrapping_add(56) as *const i16) as i64;

    if generic_container_index == -1 {
        return std::ptr::null();
    }
    *index = generic_container_index;

    let generic_container = global_metadata
        .wrapping_add(metadata_generic_containers_offset.wrapping_sub(1311232317) as usize)
        .wrapping_add(16usize * (generic_container_index as u32 as usize));

    return generic_container as *const u8;
}

pub unsafe fn il2cpp_class_get_generic_arg_count(class: *const u8) -> usize {
    let generic_class = il2cpp_class_get_generic_class(class);
    if generic_class as usize != 0 {
        let mut index = 0;
        let generic_container =
            il2cpp_generic_class_get_generic_container(generic_class, &mut index);
        if generic_container as usize != 0 {
            let idx = index as i32;
            let inner = 15660i32.wrapping_mul(idx).wrapping_add(1867048826) ^ 0x1FAF6023;
            let mid = 1168146287i32.wrapping_mul(inner) ^ 0x565C8119;
            let var_50_1 = 1360395971i32.wrapping_mul(mid);

            let generic_parameter_count = var_50_1
                ^ (*(generic_container.wrapping_add(4).cast::<i32>())).wrapping_sub(1023229573);
            return generic_parameter_count as usize;
        }
    }

    0
}

pub unsafe fn il2cpp_class_get_generic_arg_type(class: *const u8, index: usize) -> *const u8 {
    let generic_class = il2cpp_class_get_generic_class(class);
    let context_class_inst = *generic_class.wrapping_add(8).cast::<usize>() as *const u8;
    let type_argvs = *context_class_inst.wrapping_add(8).cast::<usize>() as *const u8;
    *(type_argvs.wrapping_add(index * 8).cast::<usize>()) as *const u8
}

pub unsafe fn il2cpp_class_is_generic(class: *const u8) -> bool {
    let generic_class = il2cpp_class_get_generic_class(class);
    let mut index = 0;
    (generic_class as usize) != 0
        && (il2cpp_generic_class_get_generic_container(generic_class, &mut index) as usize) != 0
}
