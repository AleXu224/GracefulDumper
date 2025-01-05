pub use crate::util::base;
use crate::util::{self, import};

import!(il2cpp_gc_disable() -> () = 0xB89A70);
import!(il2cpp_domain_get() -> usize = 0xAA7F30);
import!(il2cpp_get_corlib() -> *const u8 = 0xAA82B0);
import!(il2cpp_domain_assembly_open(domain: usize, name: *const u8) -> usize = 0xAA7F20);
import!(il2cpp_domain_get_assemblies(domain: usize, size: &mut usize) -> *const usize = 0xAA7F40);
import!(il2cpp_assembly_get_image(assembly: *const u8) -> *const u8 = 0xAA7C10);
import!(il2cpp_class_from_name(image: *const u8, namespace: *const u8, name: *const u8) -> *const u8 = 0xAA7C70);
import!(il2cpp_image_get_class_count(image: *const u8) -> usize = 0xAA82E0);
import!(il2cpp_image_get_class(image: *const u8, index: usize) -> *const u8 = 0xAA82D0);
import!(il2cpp_image_get_name(image: *const u8) -> *const i8 = 0xAA82F0);
import!(il2cpp_class_from_il2cpp_type(il2cpp_type: *const u8) -> *const u8 = 0xAA7C60);
import!(il2cpp_class_get_name(class: *const u8) -> *const i8 = 0xAA7D70);
import!(il2cpp_class_get_namespace(class: *const u8) -> *const i8 = 0xAA7D80);
import!(il2cpp_field_get_name(field: *const u8) -> *const i8 = 0xAA7F90);
import!(il2cpp_field_get_type(field: *const u8) -> *const u128 = 0xAA7FC0);
import!(il2cpp_field_get_offset(field: *const u8) -> u32 = 0xAA7FA0);
import!(il2cpp_field_get_token(field: *const u8) -> u32 = 0xAEF8C0);
import!(il2cpp_vm_class_init(class: *const u8) -> () = 0xAF2220);
import!(il2cpp_vm_class_init_methods(class: *const u8) -> () = 0xB020F0);
import!(il2cpp_type_get_assembly_qualified_name(il2cpp_type: *const u8) -> *const i8 = 0xAA86F0);
import!(il2cpp_type_get_name(il2cpp_type: *const u128) -> *const i8 = 0xAA87B0);
import!(il2cpp_field_static_get_value(field: *const u8, out: *const usize) -> () = 0xB03400);
import!(il2cpp_method_get_name(method: *const u8) -> *const i8 = 0xAEB5F0);
import!(il2cpp_class_get_method_from_name(class: *const u8, name: *const i8, args_count: i32) -> *const u8 = 0xAEB1F0);
import!(il2cpp_method_get_return_type(method: *const u8) -> *const u8 = 0xAEBC40);
import!(il2cpp_array_new(ty: *const u8, size: u32) -> *const u8 = 0xBA99E0);
import!(il2cpp_object_new(class: *const u8) -> *const u8 = 0xBA9D80);
import!(il2cpp_runtime_invoke(method: *const u8, obj: *const u8, params: *const usize, exception: &mut usize) -> usize = 0xAA8510);
import!(il2cpp_thread_attach(domain: *const u8) -> () = 0xAA86B0);

pub unsafe fn il2cpp_is_fully_initialized() -> bool {
    const G_IL2CPP_IS_FULLY_INITIALIZED: usize = 0xB225FC0;

    *(util::base().wrapping_add(G_IL2CPP_IS_FULLY_INITIALIZED) as *const u8) != 0
}

pub unsafe fn il2cpp_class_get_token(class: *const u8) -> u32 {
    *(class.wrapping_add(280) as *const u32)
}

pub unsafe fn il2cpp_method_get_arg_count(method: *const u8) -> usize {
    *method.wrapping_add(42) as usize
}

pub unsafe fn il2cpp_method_get_attrs(method: *const u8) -> u32 {
    *method.wrapping_add(38) as u32
}

pub unsafe fn il2cpp_method_get_class(method: *const u8) -> *const u8 {
    *(method.wrapping_add(8) as *const usize) as *const u8
}

pub unsafe fn il2cpp_method_get_arg_name(method: *const u8, index: usize) -> *const i8 {
    import!(il2cpp_vm_method_get_arguments(method: *const u8) -> *const u8 = 0xAEB900);
    let args = il2cpp_vm_method_get_arguments(method);
    *(args.wrapping_add(0 + 24 * index) as *const usize) as *const i8
}

pub unsafe fn il2cpp_method_get_arg_type(method: *const u8, index: usize) -> *const u128 {
    import!(il2cpp_vm_method_get_arguments(method: *const u8) -> *const u8 = 0xAEB900);
    let args = il2cpp_vm_method_get_arguments(method);
    *(args.wrapping_add(8 + 24 * index) as *const usize) as *const u128
}

pub unsafe fn il2cpp_class_get_methods(class: *const u8, count: &mut usize) -> *const usize {
    *count = *class.wrapping_add(308).cast::<u16>() as usize;
    (*class.wrapping_add(64).cast::<usize>()) as *const usize
}

pub unsafe fn il2cpp_method_get_address(method: *const u8) -> usize {
    *method.wrapping_add(0).cast::<usize>()
}

pub unsafe fn il2cpp_type_get_attrs(ty: *const u128) -> u32 {
    *(ty.wrapping_byte_add(8) as *const u32)
}

pub unsafe fn il2cpp_class_get_interfaces(class: *const u8, count: &mut usize) -> *const usize {
    *count = *(class.wrapping_add(300) as *const u16) as usize; // implementedInterfacesCount
    *(class.wrapping_add(224) as *const usize) as *const usize
}

pub unsafe fn il2cpp_class_get_type(class: *const u8) -> *const u128 {
    class.wrapping_add(160).cast::<u128>() // in-place struct
}

pub unsafe fn il2cpp_class_get_image(class: *const u8) -> *const u8 {
    *(class.wrapping_add(200) as *const usize) as *const u8
}

pub unsafe fn il2cpp_class_get_fields(class: *const u8, count: &mut usize) -> *const u8 {
    *count = *(class.wrapping_add(296) as *const u16) as usize;
    *(class.wrapping_add(216) as *const usize) as *const u8
}

pub unsafe fn il2cpp_class_get_generic_class(class: *const u8) -> *const u8 {
    (*class.wrapping_add(40).cast::<usize>()) as *const u8
}

pub unsafe fn il2cpp_class_get_parent(class: *const u8) -> *const u8 {
    (*class.wrapping_add(104).cast::<usize>()) as *const u8
}

pub unsafe fn il2cpp_generic_class_get_generic_container(generic_class: *const u8) -> *const u8 {
    (*generic_class.wrapping_add(8).cast::<usize>()) as *const u8
}

pub unsafe fn il2cpp_class_get_generic_arg_count(class: *const u8) -> usize {
    let generic_class = il2cpp_class_get_generic_class(class);
    if (generic_class as usize) != 0 {
        let generic_container = il2cpp_generic_class_get_generic_container(generic_class);
        if (generic_container as usize) != 0 {
            return *generic_container.cast::<u32>() as usize;
        }
    }

    0
}

pub unsafe fn il2cpp_class_get_generic_arg_type(class: *const u8, index: usize) -> *const u8 {
    let generic_class = il2cpp_class_get_generic_class(class);
    let generic_container = il2cpp_generic_class_get_generic_container(generic_class);

    let argv = *generic_container.wrapping_add(8).cast::<usize>() as *const usize;
    *(argv.wrapping_byte_add(index * 8)) as *const u8
}

pub unsafe fn il2cpp_class_is_generic(class: *const u8) -> bool {
    let generic_class = il2cpp_class_get_generic_class(class);
    (generic_class as usize) != 0
        && (il2cpp_generic_class_get_generic_container(generic_class) as usize) != 0
}
