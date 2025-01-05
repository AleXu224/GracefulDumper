use std::{borrow::Cow, sync::OnceLock};

use windows::{core::s, Win32::System::LibraryLoader::LoadLibraryA};

static BASE: OnceLock<usize> = OnceLock::new();

#[inline]
pub fn base() -> usize {
    *BASE.get_or_init(|| unsafe { LoadLibraryA(s!("GameAssembly.dll")).unwrap().0 as usize })
}

#[inline]
pub unsafe fn cstr(s: *const i8) -> Cow<'static, str> {
    std::ffi::CStr::from_ptr(s).to_string_lossy()
}

macro_rules! import {
    ($name:ident($($arg_name:ident: $arg_type:ty),*) -> $ret_type:ty = $rva:expr) => {
        pub unsafe fn $name($($arg_name: $arg_type,)*) -> $ret_type {
            type FuncType = unsafe extern "fastcall" fn($($arg_type,)*) -> $ret_type;
            ::std::mem::transmute::<usize, FuncType>(crate::util::base() + $rva)($($arg_name,)*)
        }
    };
}

macro_rules! as_cstr {
    ($s:expr) => {
        ::std::ffi::CString::new($s)
            .unwrap()
            .to_bytes_with_nul()
            .as_ptr()
    };
}

pub(crate) use as_cstr;
pub(crate) use import;
