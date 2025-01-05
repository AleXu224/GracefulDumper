use super::*;

#[repr(transparent)]
pub struct Assembly(*const u8);

impl From<*const u8> for Assembly {
    fn from(value: *const u8) -> Self {
        (value as usize != 0)
            .then_some(Self(value))
            .expect("Assembly::from(null)")
    }
}

impl Assembly {
    pub fn image(&self) -> Il2cppImage {
        unsafe { Il2cppImage::from(il2cpp_assembly_get_image(self.0)) }
    }
}
