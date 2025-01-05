mod array;
mod assembly;
pub mod attributes;
mod class;
mod domain;
mod exception;
mod field;
mod image;
mod method;
mod object;
mod string;

use crate::ffi::*;
use crate::util::{as_cstr, cstr};
use std::borrow::Cow;
use std::mem;

pub use array::Il2cppArray;
pub use assembly::Assembly;
pub use class::Il2cppClass;
pub use domain::Il2cppDomain;
pub use exception::Il2cppException;
pub use field::Il2cppField;
pub use image::Il2cppImage;
pub use method::Il2cppMethod;
pub use object::Il2cppObject;
pub use string::Il2cppString;

pub trait Il2cppValue {
    fn as_raw(&self) -> usize;
}

impl<T: Copy + Into<usize>> Il2cppValue for T {
    fn as_raw(&self) -> usize {
        (*self).into()
    }
}

#[repr(transparent)]
pub struct Il2cppType(*const u128);

impl Il2cppType {
    pub fn name(&self) -> Cow<'static, str> {
        unsafe { cstr(il2cpp_type_get_name(self.0)) }
    }

    pub fn attrs(&self) -> u32 {
        unsafe { il2cpp_type_get_attrs(self.0) }
    }

    pub fn type_enum(&self) -> u8 {
        unsafe { *self.0.cast::<u8>().wrapping_add(10) }
    }

    pub fn to_class(&self) -> Il2cppClass {
        unsafe { Il2cppClass(il2cpp_class_from_il2cpp_type(self.0 as *const u8)) }
    }
}

pub struct Void;

impl From<usize> for Void {
    fn from(_: usize) -> Self {
        Self
    }
}
