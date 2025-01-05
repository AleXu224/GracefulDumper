use windows::Win32::{
    Foundation::{HANDLE, HMODULE},
    System::{
        Diagnostics::Debug::{
            WriteProcessMemory, IMAGE_DIRECTORY_ENTRY_BASERELOC, IMAGE_NT_HEADERS64,
        },
        Memory::{VirtualAlloc, VirtualAllocEx, MEM_COMMIT, PAGE_EXECUTE_READWRITE},
        SystemServices::{IMAGE_BASE_RELOCATION, IMAGE_DOS_HEADER},
    },
};

pub unsafe fn relocate_image(source: HMODULE, target_process: HANDLE) -> usize {
    let dos_header = source.0 as *const IMAGE_DOS_HEADER;
    let nt_header =
        source.0.wrapping_add((*dos_header).e_lfanew as usize) as *const IMAGE_NT_HEADERS64;

    let size_of_image = (*nt_header).OptionalHeader.SizeOfImage as usize;

    let local_image = VirtualAlloc(None, size_of_image, MEM_COMMIT, PAGE_EXECUTE_READWRITE);
    std::ptr::copy_nonoverlapping(source.0 as *const u8, local_image as *mut u8, size_of_image);

    let target_image = VirtualAllocEx(
        target_process,
        None,
        size_of_image,
        MEM_COMMIT,
        PAGE_EXECUTE_READWRITE,
    );

    let delta_image_base = target_image.wrapping_sub(source.0 as usize);
    let mut relocation_table = local_image.wrapping_add(
        (*nt_header).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_BASERELOC.0 as usize]
            .VirtualAddress as usize,
    ) as *const IMAGE_BASE_RELOCATION;

    while (*relocation_table).SizeOfBlock > 0 {
        let relocation_entries_count = ((*relocation_table).SizeOfBlock as usize
            - std::mem::size_of::<IMAGE_BASE_RELOCATION>())
            / std::mem::size_of::<u16>();

        let relocation_rva =
            ((relocation_table as usize) + size_of::<IMAGE_BASE_RELOCATION>()) as *const u16;

        for i in 0..relocation_entries_count {
            if (*(relocation_rva.wrapping_add(i)) & 0xFFF) != 0 {
                let patched_address = local_image
                    .wrapping_add((*relocation_table).VirtualAddress as usize)
                    .wrapping_add((*(relocation_rva.wrapping_add(i)) & 0xFFF) as usize)
                    as *mut usize;

                *patched_address += delta_image_base as usize;
            }
        }

        relocation_table = ((relocation_table as usize) + (*relocation_table).SizeOfBlock as usize)
            as *const IMAGE_BASE_RELOCATION;
    }

    WriteProcessMemory(
        target_process,
        target_image,
        local_image,
        (*nt_header).OptionalHeader.SizeOfImage as usize,
        None,
    )
    .unwrap();

    delta_image_base as usize
}
