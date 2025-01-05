use super::*;

#[repr(transparent)]
pub struct Il2cppField([u8; 32]);

impl Il2cppField {
    pub fn name(&self) -> Cow<'static, str> {
        unsafe { cstr(il2cpp_field_get_name(self.0.as_ptr())) }
    }

    pub fn il2cpp_type(&self) -> Il2cppType {
        unsafe { Il2cppType(il2cpp_field_get_type(self.0.as_ptr())) }
    }

    pub fn offset(&self) -> u32 {
        unsafe { il2cpp_field_get_offset(self.0.as_ptr()) }
    }

    pub fn token(&self) -> u32 {
        unsafe { il2cpp_field_get_token(self.0.as_ptr()) }
    }

    pub fn is_instance(&self) -> bool {
        self.il2cpp_type().attrs() & attributes::FIELD_ATTRIBUTE_STATIC == 0
    }

    pub fn static_get_value(&self) -> usize {
        unsafe {
            let mut out = 0;
            il2cpp_field_static_get_value(self.0.as_ptr(), &mut out);
            out
        }
    }

    pub fn get_field_data_ptr(&self, obj: &Il2cppObject) -> *const u8 {
        obj.0.wrapping_add(self.offset() as usize)
    }
}
