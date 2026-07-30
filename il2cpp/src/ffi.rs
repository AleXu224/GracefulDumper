pub use crate::util::base;
use crate::util::{self, import};

pub const GLOBAL_METADATA_HEADER: usize = 0x50B72E0;
pub const GLOBAL_METADATA: usize = 0x50B72E8;

import!(il2cpp_gc_disable() -> () = 0xACE310);
import!(il2cpp_domain_get() -> usize = 0xACDE10);
import!(il2cpp_get_corlib() -> *const u8 = 0xACD740);
import!(il2cpp_domain_assembly_open(domain: usize, name: *const u8) -> usize = 0xACDE40);
import!(il2cpp_domain_get_assemblies(domain: usize, size: &mut usize) -> *const usize = 0xACDE50); // 48 8B 05 ? ? ? ? 48 29 C1
import!(il2cpp_assembly_get_image(assembly: *const u8) -> *const u8 = 0xACD980);
import!(il2cpp_class_from_name(image: *const u8, namespace: *const u8, name: *const u8) -> *const u8 = 0xACDA10);
import!(il2cpp_image_get_class_count(image: *const u8) -> usize = 0xACF010);
import!(il2cpp_image_get_class(image: *const u8, index: usize) -> *const u8 = 0xACF020);
import!(il2cpp_image_get_name(image: *const u8) -> *const i8 = 0xACF000);
import!(il2cpp_class_from_il2cpp_type(il2cpp_type: *const u8) -> *const u8 = 0x264420);
// import!(il2cpp_class_get_name(class: *const u8) -> *const i8 = 0xAA7D70);
// import!(il2cpp_class_get_namespace(class: *const u8) -> *const i8 = 0xAA7D80);
import!(il2cpp_field_get_name(field: *const u8) -> *const i8 = 0xACE050);
import!(il2cpp_field_get_type(field: *const u8) -> *const u128 = 0xACE0A0);
import!(il2cpp_field_get_offset(field: *const u8) -> u32 = 0xACE090);
// import!(il2cpp_field_get_token(field: *const u8) -> u32 = 0xAEF8C0);
import!(il2cpp_vm_class_init(class: *const u8) -> () = 0x258440); // 48 C7 45 ? ? ? ? ? F6 81
import!(il2cpp_vm_class_init_methods(class: *const u8) -> () = 0x26D6E0); // 48 C7 45 ? ? ? ? ? 31 C0 48 85 C9
                                                                          // import!(il2cpp_type_get_assembly_qualified_name(il2cpp_type: *const u8) -> *const i8 = 0xAA86F0);
import!(il2cpp_type_get_name(il2cpp_type: *const u128) -> *const i8 = 0xACED20); // 48 C7 45 ? ? ? ? ? 48 89 CA 0F 57 C0 0F 29 45 ? 48 C7 45 ? ? ? ? ? 48 C7 45 ? ? ? ? ? 48 8D 7D ? 48 89 F9 45 31 C0
import!(il2cpp_field_static_get_value(field: *const u8, out: *const usize) -> () = 0x272050);
import!(il2cpp_method_get_name(method: *const u8) -> *const i8 = 0x27D200);
import!(il2cpp_class_get_method_from_name(class: *const u8, name: *const i8, args_count: i32) -> *const u8 = 0xACDB00);
import!(il2cpp_method_get_return_type(method: *const u8) -> *const u8 = 0x27D300); // 48 B9 ? ? ? ? ? ? ? ? ? ? ? 48 B9 ? ? ? ? ? ? ? ? 48 89 48 ? 48 B9 ? ? ? ? ? ? ? ? 48 89 48 ? 48 89 46
import!(il2cpp_array_new(ty: *const u8, size: u64) -> *const u8 = 0x2582E0);
import!(il2cpp_object_new(class: *const u8) -> *const u8 = 0x27EDD0); // 48 C7 45 ? ? ? ? ? 48 89 CE F6 81 ? ? ? ? ? 75 ? 48 8D 05 ? ? ? ? 48 89 45 ? 48 8B 0D ? ? ? ? FF 15 ? ? ? ? 48 8D 55 ? 48 89 F1 E8 ? ? ? ? 48 8B 45 ? ? ? ? FF 15 ? ? ? ? 48 8B 46
import!(il2cpp_runtime_invoke(method: *const u8, obj: *const u8, params: *const usize, exception: &mut usize) -> usize = 0x281A30); // 48 83 EC ? 48 8D 6C 24 ? 48 C7 45 ? ? ? ? ? 4C 89 C6 48 89 D3 48 89 CF 4D 85 C9
import!(il2cpp_thread_attach(domain: *const u8) -> () = 0x28A800); // 48 C7 45 ? ? ? ? ? 48 89 CE 48 8B 05 ? ? ? ? ? ? FF 15

import!(method_info_from_index(index: u32) -> * const u8 = 0x27ACC0); // 81 E1 ? ? ? ? 74 ? 25
import!(string_literal_from_index(index: u32, param1: *const u8, param2: *const u8) -> * const u8 = 0x27B370);

// import!(initialize_by_index(index: u32) -> () = 0x265630);

import!(metadatacache_getstringfromindex(param: u32) -> *const i8 = 0x265B20);

// Had no implementation in the binary
pub unsafe fn il2cpp_field_get_token(field: *const u8) -> u32 {
    *(field.wrapping_add(28) as *const u32) ^ 0x1428D8E1
}

pub unsafe fn il2cpp_class_get_name(class: *const u8) -> *const i8 {
    *(class.wrapping_add(80) as *const usize) as *const i8
}

pub unsafe fn il2cpp_is_fully_initialized() -> bool {
    const S_INITIALIZEDIL2CPPFROMWINDOWSRUNTIME: usize = 0x50B72B8;

    *(util::base().wrapping_add(S_INITIALIZEDIL2CPPFROMWINDOWSRUNTIME) as *const u8) != 0
}

pub unsafe fn il2cpp_ptr_base() -> u64 {
    // A 48 Byte struct heap allocated struct that is initialized in Runtime::Init() that holds an arary that is used everywhere
    // The first field seems to be
    const STRUCT_OFFSET: usize = 0x50B6E90;

    let struct_ptr = util::base().wrapping_add(STRUCT_OFFSET) as *const u64;
    let first_field_ptr = *struct_ptr.wrapping_add(0) as *const u64;

    *first_field_ptr
}

pub unsafe fn il2cpp_class_get_token(class: *const u8) -> u32 {
    *(class.wrapping_add(160) as *const u32)
}

pub unsafe fn il2cpp_method_get_arg_count(method: *const u8) -> usize {
    *method.wrapping_add(50) as usize
}

pub unsafe fn il2cpp_method_get_attrs(method: *const u8) -> u32 {
    *method.wrapping_add(44) as u32
}

pub unsafe fn il2cpp_method_get_class(method: *const u8) -> *const u8 {
    *(method.wrapping_add(0) as *const usize) as *const u8
}

pub unsafe fn il2cpp_method_get_arg_name(method: *const u8, index: usize) -> *const i8 {
    import!(il2cpp_vm_method_get_arguments(method: *const u8) -> *const u8 = 0x27D550);
    let args = il2cpp_vm_method_get_arguments(method);
    *(args.wrapping_add(0 + 24 * index) as *const usize) as *const i8
}

pub unsafe fn il2cpp_method_get_arg_type(method: *const u8, index: usize) -> *const u128 {
    import!(il2cpp_vm_method_get_arguments(method: *const u8) -> *const u8 = 0x27D550);
    let args = il2cpp_vm_method_get_arguments(method);
    *(args.wrapping_add(8 + 24 * index) as *const usize) as *const u128
}

pub unsafe fn il2cpp_class_get_methods(class: *const u8, count: &mut usize) -> *const usize {
    *count = *class.wrapping_add(196).cast::<u16>() as usize;
    (*class.wrapping_add(48).cast::<usize>()) as *const usize
}

pub unsafe fn il2cpp_method_get_address(method: *const u8) -> usize {
    *method.wrapping_add(8).cast::<usize>()
}

pub unsafe fn il2cpp_type_get_attrs(ty: *const u128) -> u32 {
    *(ty.wrapping_byte_add(8) as *const u32)
}

pub unsafe fn il2cpp_class_get_interfaces(class: *const u8, count: &mut usize) -> *const usize {
    *count = *(class.wrapping_add(60) as *const u16) as usize; // implementedInterfacesCount
    *(class.wrapping_add(64) as *const usize) as *const usize
}

pub unsafe fn il2cpp_class_get_type_defition(class: *const u8) -> *const u8 {
    *(class.wrapping_add(88) as *const usize) as *const u8
}

pub unsafe fn il2cpp_type_definition_get_namespace_index(type_definition: *const u8) -> u32 {
    *(type_definition.wrapping_add(8) as *const u32)
}

pub unsafe fn il2cpp_type_definition_get_field_count(type_definition: *const u8) -> u16 {
    *(type_definition.wrapping_add(52) as *const u16)
}

pub unsafe fn il2cpp_class_get_type(class: *const u8) -> *const u128 {
    class.wrapping_add(128).cast::<u128>() // in-place struct
}

pub unsafe fn il2cpp_class_get_image(class: *const u8) -> *const u8 {
    *(class.wrapping_add(0) as *const usize) as *const u8
}

pub unsafe fn il2cpp_class_get_fields(class: *const u8) -> *const u8 {
    *(class.wrapping_add(24) as *const usize) as *const u8
}

pub unsafe fn il2cpp_class_get_generic_class(class: *const u8) -> *const u8 {
    (*class.wrapping_add(56).cast::<usize>()) as *const u8
}

pub unsafe fn il2cpp_class_get_parent(class: *const u8) -> *const u8 {
    let parent_offset = *class.wrapping_add(176).cast::<u32>();
    if parent_offset == 0 {
        return std::ptr::null();
    }
    let parent_array = il2cpp_ptr_base();
    (parent_offset as u64 + parent_array) as *const u8
}

pub unsafe fn il2cpp_generic_class_get_generic_container(
    generic_class: *const u8,
    index: &mut i64,
) -> *const u8 {
    let type_definition_index = *generic_class.wrapping_add(0).cast::<u32>();
    let global_metadata_header = *((base() + GLOBAL_METADATA_HEADER) as *const usize);
    let global_metadata = *((base() + GLOBAL_METADATA) as *const usize);

    let metadata_type_definitions_offset =
        *(global_metadata_header.wrapping_add(232) as *const i32) as i64;
    let metadata_generic_containers_offset =
        *(global_metadata_header.wrapping_add(172) as *const i32);

    let type_definition = global_metadata
        .wrapping_add((metadata_type_definitions_offset ^ 0x2C8195F3) as usize)
        .wrapping_add(80usize * (type_definition_index as usize));
    let generic_container_index = *(type_definition.wrapping_add(54) as *const u16) as i64;

    if generic_container_index == -1 {
        return std::ptr::null();
    }
    *index = generic_container_index;

    let generic_container = global_metadata
        .wrapping_add((metadata_generic_containers_offset.wrapping_add( 0xf1058adb as i64 as i32)) as usize)
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
            let var_50_1 = ((((0x78172412eba485 + 0x190bab230232 * index) ^ 0x7b942d24)
                * 0x3df9dfe7)
                >> 0x17) as i32
                ^ 0x77a77aa9;

            let generic_parameter_count =
                *generic_container.wrapping_add(4).cast::<i32>() ^ var_50_1 ^ 0x2c7623b8;
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
