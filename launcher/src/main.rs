use std::ffi::c_void;
use std::fs::File;
use std::io::{BufWriter, Write};

use std::ptr::null_mut;
use windows::core::{s, PCSTR, PSTR};
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Console::AllocConsole;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::Threading::{
    CreateProcessA, CreateRemoteThread, ResumeThread, WaitForSingleObject, CREATE_SUSPENDED,
    PROCESS_INFORMATION, STARTUPINFOA,
};

mod reloc;
mod util;

const GAME_EXECUTABLE: PCSTR = s!("ZenlessZoneZero.exe");

unsafe fn inject_self(target: HANDLE) {
    let self_base = GetModuleHandleA(PCSTR::null()).unwrap();
    let reloc_delta = reloc::relocate_image(self_base, target);

    let h_thread = CreateRemoteThread(
        target,
        None,
        0,
        Some(std::mem::transmute((entry_point as usize).wrapping_add(reloc_delta))),
        None,
        0,
        None,
    )
    .unwrap();

    WaitForSingleObject(h_thread, 0xFFFFFFFF);
    CloseHandle(h_thread).unwrap();
}

unsafe fn dump_thread() {
    use std::time::Duration;

    println!("ReversedRooms™ GracefulDumper for ZZZ 0.2.0");
    println!("Copyright xeondev, 2025. All bytes reversed.");

    while !il2cpp::ffi::il2cpp_is_fully_initialized() {
        std::thread::sleep(Duration::from_millis(100));
    }

    println!("il2cpp is fully initialized now, time to dump!");

    print!("Generating dump.cs...");
    std::io::stdout().flush().unwrap();
    let mut dump_cs = File::create("dump.cs").unwrap();
    dumpcs_gen::dump(&mut BufWriter::new(&mut dump_cs)).unwrap();
    println!("done!");

    print!("Generating nap.proto and nap.json...");
    std::io::stdout().flush().unwrap();
    let proto_file = proto_gen::dump().unwrap();
    let mut nap_proto = File::create("nap.proto").unwrap();
    writeln!(BufWriter::new(&mut nap_proto), "{proto_file}").unwrap();
    let mut nap_json = File::create("nap.json").unwrap();
    proto_file.write_json(&mut BufWriter::new(&mut nap_json)).unwrap();
    println!("done!");

    println!("dump finished!");
}

unsafe extern "system" fn entry_point(_: *mut c_void) -> u32 {
    if util::is_wine() {
        util::patch_wintrust();
    }

    let _ = AllocConsole();
    std::thread::spawn(|| unsafe {
        dump_thread();
    });

    0
}

fn main() {
    let mut proc_info = PROCESS_INFORMATION::default();
    let mut startup_info = STARTUPINFOA::default();

    unsafe {
        CreateProcessA(
            GAME_EXECUTABLE,
            PSTR(null_mut()),
            None,
            None,
            false,
            CREATE_SUSPENDED,
            None,
            None,
            &mut startup_info,
            &mut proc_info,
        )
        .unwrap();

        inject_self(proc_info.hProcess);
        ResumeThread(proc_info.hThread);

        CloseHandle(proc_info.hThread).unwrap();
        CloseHandle(proc_info.hProcess).unwrap();
    }
}
