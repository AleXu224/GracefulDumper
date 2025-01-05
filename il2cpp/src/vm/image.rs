use super::*;

#[repr(transparent)]
pub struct Il2cppImage(*const u8);

impl From<*const u8> for Il2cppImage {
    fn from(value: *const u8) -> Self {
        (value as usize != 0)
            .then_some(Self(value))
            .expect("Il2cppImage::from(null)")
    }
}

impl Il2cppImage {
    pub fn name(&self) -> Cow<'static, str> {
        unsafe { cstr(il2cpp_image_get_name(self.0)) }
    }

    pub fn get_class_count(&self) -> usize {
        unsafe { il2cpp_image_get_class_count(self.0) }
    }

    pub fn get_class(&self, index: usize) -> Il2cppClass {
        unsafe { Il2cppClass(il2cpp_image_get_class(self.0, index)) }
    }

    pub fn get_class_by_name(&self, namespace: &str, name: &str) -> Option<Il2cppClass> {
        let ptr = unsafe { il2cpp_class_from_name(self.0, as_cstr!(namespace), as_cstr!(name)) };
        ((ptr as usize) != 0).then_some(Il2cppClass(ptr))
    }
}
