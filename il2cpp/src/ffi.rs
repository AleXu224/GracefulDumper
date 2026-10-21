pub use crate::util::base;
use crate::util::{self, import};

pub const GLOBAL_METADATA_HEADER: usize = 0x4E9CD90;
pub const GLOBAL_METADATA: usize = 0x4E9CD98;

import!(il2cpp_gc_disable() -> () = 0x9B4AF0); // 74 ? ? ? ? ? ? ? ? FF 15 ? ? ? ? ? ? ? ? ? ? ? FF 05 ? ? ? ? ? ? 75
import!(il2cpp_domain_get() -> usize = 0x9B45D0); // 48 8B 05 ? ? ? ? 48 85 C0 74 ? 48 83 C4 28 C3
import!(il2cpp_domain_assembly_open(domain: usize, name: *const u8) -> usize = 0x9B4600); // ".dll" search
import!(il2cpp_domain_get_assemblies(domain: usize, size: &mut usize) -> *const usize = 0x9B4610); // 48 8B 05 ? ? ? ? 48 29 C1
import!(il2cpp_assembly_get_image(assembly: *const u8) -> *const u8 = 0x9B4130); // In Runtime::Init right at "mscorlib.dll"
import!(il2cpp_class_from_name(image: *const u8, namespace: *const u8, name: *const u8) -> *const u8 = 0x9B41C0); // function that jumps to 48 C7 45 ? ? ? ? ? 48 89 55 ? ? ? ? ? ? ? ? ? ? ? ? ? ? ? ? ? ? 4C 89 45
import!(il2cpp_image_get_class_count(image: *const u8) -> usize = 0x9B5820); // Can find the XOR constant inside Image::ClassFromName from il2cpp_class_from_name
import!(il2cpp_image_get_class(image: *const u8, index: usize) -> *const u8 = 0x9B5830); // Just under il2cpp_image_get_class_count
import!(il2cpp_image_get_name(image: *const u8) -> *const i8 = 0x9B5810); // Just above il2cpp_image_get_class_count
import!(il2cpp_class_from_il2cpp_type(il2cpp_type: *const u8) -> *const u8 = 0x28C9D0); // Find "GenericClass" in Runtime::Init and get the global that it initializes, if you xref that then the next function is gonna be this
import!(il2cpp_field_get_name(field: *const u8) -> *const i8 = 0x9B4810); // Find inside Enum::GetEnumValuesAndNames, next to "value__"
import!(il2cpp_field_get_type(field: *const u8) -> *const u128 = 0x9B4860); // get type XOR from InitLocked > SetupFieldsLocked > SetupFieldsFromDefinitionLocked. Should be just above the name usage, can find the other fieldinfo members here as well
import!(il2cpp_field_get_offset(field: *const u8) -> u32 = 0x9B4850); // Right next to the previous two functions. Has an unique signature, should be easy to spot.
import!(il2cpp_vm_class_init(class: *const u8) -> () = 0x280B30); // 48 C7 45 ? ? ? ? ? F6 81
import!(il2cpp_vm_class_init_methods(class: *const u8) -> () = 0x2958C0); // 48 C7 45 ? ? ? ? ? 31 C0 48 85 C9
import!(il2cpp_type_get_name(il2cpp_type: *const u128) -> *const i8 = 0x9B5530); // 48 C7 45 ? ? ? ? ? 48 89 CA 0F 57 C0 0F 29 45 ? 48 C7 45 ? ? ? ? ? 48 C7 45 ? ? ? ? ? 48 8D 7D ? 48 89 F9 45 31 C0
import!(il2cpp_field_static_get_value(field: *const u8, out: *const usize) -> () = 0x29A090); // Find xref of SetupFieldsLocked
import!(il2cpp_method_get_name(method: *const u8) -> *const i8 = 0x2A5460); // Find FormatExceptionMessageForNonGenericMethod using "is not a generic method" and get the xor that can be found right after the "::" lines
import!(il2cpp_class_get_method_from_name(class: *const u8, name: *const i8, args_count: i32) -> *const u8 = 0x9B42B0); // CreateUnhandledExceptionEventArgs > Go to the supposed Class::GetMethodFromNameFlags > xref the function between the mutex > the first function should be what you are looking for > xref that function and find the usage with two zeros for the last two params
import!(il2cpp_method_get_return_type(method: *const u8) -> *const u8 = 0x2A5570); // 48 B9 ? ? ? ? ? ? ? ? ? ? ? 48 B9 ? ? ? ? ? ? ? ? 48 89 48 ? 48 B9 ? ? ? ? ? ? ? ? 48 89 48 ? 48 89 46
import!(il2cpp_array_new(ty: *const u8, size: u64) -> *const u8 = 0x2809D0); // Second xref on InitLocked
import!(il2cpp_object_new(class: *const u8) -> *const u8 = 0x2A7080); // 48 C7 45 ? ? ? ? ? 48 89 CE F6 81 ? ? ? ? ? 75 ? 48 8D 05 ? ? ? ? 48 89 45 ? 48 8B 0D ? ? ? ? FF 15 ? ? ? ? 48 8D 55 ? 48 89 F1 E8 ? ? ? ? 48 8B 45 ? ? ? ? FF 15 ? ? ? ? 48 8B 46
import!(il2cpp_runtime_invoke(method: *const u8, obj: *const u8, params: *const usize, exception: &mut usize) -> usize = 0x2A9C50); // 48 83 EC ? 48 8D 6C 24 ? 48 C7 45 ? ? ? ? ? 4C 89 C6 48 89 D3 48 89 CF 4D 85 C9
import!(il2cpp_thread_attach(domain: *const u8) -> () = 0x2B2A60); // 48 C7 45 ? ? ? ? ? 48 89 CE 48 8B 05 ? ? ? ? ? ? FF 15

import!(metadatacache_getstringfromindex(param: u32) -> *const i8 = 0x28E0C0); // Inside FromTypeDefinition from InitLocked

// Had no implementation in the binary
pub unsafe fn il2cpp_field_get_token(field: *const u8) -> u32 {
    // Found in SetupFieldsLocked in the same way as the other three
    // InitLocked > SetupFieldsLocked > SetupFieldsFromDefinitionLocked matches the code order pretty nicely and should yield good results
    *(field.wrapping_add(24) as *const u32) ^ 0x1EB4E5CB
}

pub unsafe fn il2cpp_class_get_name(class: *const u8) -> *const i8 {
    // Inside InitLocked when creating the error message
    *(class.wrapping_add(0x48) as *const usize) as *const i8
}

pub unsafe fn il2cpp_is_fully_initialized() -> bool {
    const S_INITIALIZEDIL2CPPFROMWINDOWSRUNTIME: usize = 0x4E9CD68;

    *(util::base().wrapping_add(S_INITIALIZEDIL2CPPFROMWINDOWSRUNTIME) as *const u8) != 0
}

pub unsafe fn il2cpp_ptr_base() -> u64 {
    // A 48 Byte struct heap allocated struct that is initialized in Runtime::Init() that holds an arary that is used everywhere
    // The first field seems to be
    const STRUCT_OFFSET: usize = 0x4E9C940;

    let struct_ptr = util::base().wrapping_add(STRUCT_OFFSET) as *const u64;
    let first_field_ptr = *struct_ptr.wrapping_add(0) as *const u64;

    *first_field_ptr
}

pub unsafe fn il2cpp_class_get_token(class: *const u8) -> u32 {
    // InitLocked > SetupInterfacesLocked > fromIl2CppType > GenericClass::GetClass
    *(class.wrapping_add(0xA4) as *const u32)
}

pub unsafe fn il2cpp_method_get_arg_count(method: *const u8) -> usize {
    // Inside Class::GetMethodFromNameFlags
    *method.wrapping_add(50) as usize // Stable 3.0 - 3.3 possibly not changing?
}

pub unsafe fn il2cpp_method_get_flags(method: *const u8) -> u32 {
    *method.wrapping_add(0x2E) as u32 // Inside SetupMethodsLocked, has a really unique assignment code, compare to prev version
}

pub unsafe fn il2cpp_method_get_class(method: *const u8) -> *const u8 {
    *(method.wrapping_add(8) as *const usize) as *const u8 // Same as flags, the member which has the arg assigned to it
}

import!(il2cpp_vm_method_get_arguments(method: *const u8) -> *const u8 = 0x2A57E0); // Found inside PlatformInvoke::MarshalDelegate, also 4C 8B 05 ? ? ? ? 4C 39 C2
pub unsafe fn il2cpp_method_get_arg_name(method: *const u8, index: usize) -> *const i8 {
    let args = il2cpp_vm_method_get_arguments(method);
    *(args.wrapping_add(0 + 24 * index) as *const usize) as *const i8
}

pub unsafe fn il2cpp_method_get_arg_type(method: *const u8, index: usize) -> *const u128 {
    let args = il2cpp_vm_method_get_arguments(method);
    *(args.wrapping_add(8 + 24 * index) as *const usize) as *const u128
}

pub unsafe fn il2cpp_class_get_methods(class: *const u8, count: &mut usize) -> *const usize {
    *count = *class.wrapping_add(0xC4).cast::<u16>() as usize;
    (*class.wrapping_add(0x28).cast::<usize>()) as *const usize
}

pub unsafe fn il2cpp_method_get_address(method: *const u8) -> usize {
    *method.wrapping_add(0).cast::<usize>()
}

pub unsafe fn il2cpp_type_get_attrs(ty: *const u128) -> u32 {
    *(ty.wrapping_byte_add(8) as *const u32)
}

pub unsafe fn il2cpp_class_get_interfaces(class: *const u8, count: &mut usize) -> *const usize {
    // SetupInterfacesLocked
    let type_definition = (class.wrapping_add(0x50) as *const usize).read() as *const u8;
    *count = *(type_definition.wrapping_add(0x38) as *const u16) as usize;
    *(class.wrapping_add(0x68) as *const usize) as *const usize
}

pub unsafe fn il2cpp_class_get_type_defition(class: *const u8) -> *const u8 {
    // SetupInterfacesLocked
    *(class.wrapping_add(0x50) as *const usize) as *const u8
}

pub unsafe fn il2cpp_type_definition_get_namespace_index(type_definition: *const u8) -> u32 {
    // InitLocked inside the error string creation
    *(type_definition.wrapping_add(24) as *const u32)
}

pub unsafe fn il2cpp_type_definition_get_field_count(type_definition: *const u8) -> u16 {
    // InitLocked > SetupFieldsLocked > SetupFieldsFromDefinitionLocked
    *(type_definition.wrapping_add(0x42) as *const u16)
}

pub unsafe fn il2cpp_class_get_type(class: *const u8) -> *const u128 {
    // InitLocked above SetupInterfacesLocked, make sure not to get byval_arg submembers by mistake
    class.wrapping_add(0x80).cast::<u128>() // byval_arg
}

pub unsafe fn il2cpp_class_get_image(class: *const u8) -> *const u8 {
    // FromTypeDefinition
    *(class.wrapping_add(0) as *const usize) as *const u8
}

pub unsafe fn il2cpp_class_get_fields(class: *const u8) -> *const u8 {
    // InitLocked > SetupFieldsLocked > SetupFieldsFromDefinitionLocked
    *(class.wrapping_add(0x60) as *const usize) as *const u8
}

pub unsafe fn il2cpp_class_get_generic_class(class: *const u8) -> *const u8 {
    (*class.wrapping_add(0x20).cast::<usize>()) as *const u8
}

pub unsafe fn il2cpp_class_get_parent(class: *const u8) -> *const u8 {
    let parent_offset = *class.wrapping_add(0xA0).cast::<u32>();
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
        *(global_metadata_header.wrapping_add(0x30) as *const u32);
    let metadata_generic_containers_offset =
        *(global_metadata_header.wrapping_add(0x98) as *const u32);

    let type_definition = global_metadata
        .wrapping_add(metadata_type_definitions_offset.wrapping_add(0xEB07C176) as i32 as usize)
        .wrapping_add(0x50usize * (type_definition_index as usize));
    let generic_container_index = *(type_definition.wrapping_add(0x44) as *const i16) as i64;

    if generic_container_index == -1 {
        return std::ptr::null();
    }
    *index = generic_container_index;

    let generic_container = global_metadata
        .wrapping_add(metadata_generic_containers_offset.wrapping_add(0xC9E008A1) as i32 as usize)
        .wrapping_add(0x10usize * (generic_container_index as u32 as usize));

    return generic_container as *const u8;
}

pub unsafe fn il2cpp_class_get_generic_arg_count(class: *const u8) -> usize {
    let generic_class = il2cpp_class_get_generic_class(class);
    if generic_class as usize != 0 {
        let mut index = 0;
        let generic_container =
            il2cpp_generic_class_get_generic_container(generic_class, &mut index);
        if generic_container as usize != 0 {
            let idx = index as i16 as i32;
            let hash = idx.wrapping_mul(0x268B) ^ 0x17DE0D96;
            let hash = hash.wrapping_mul(0x326F94C9) ^ 0x43651645;
            let hash = hash.wrapping_add(0x39AE049B);

            let generic_parameter_count = (*(generic_container.wrapping_add(0xC).cast::<i32>()))
                .wrapping_sub(hash)
                .wrapping_add(0xE0F271F9u32 as i32);
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
