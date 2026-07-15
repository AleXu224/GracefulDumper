use super::*;

#[repr(transparent)]
pub struct Il2cppString(pub *const u8);

impl Il2cppValue for Il2cppString {
    fn as_raw(&self) -> usize {
        self.0 as usize
    }
}

impl std::fmt::Display for Il2cppString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<usize> for Il2cppString {
    fn from(value: usize) -> Self {
        Self(value as *const u8)
    }
}

impl Il2cppString {
    pub fn len(&self) -> usize {
        if self.0 as usize == 0 {
            return 0;
        }
        unsafe { *self.0.wrapping_add(16).cast::<u32>() as usize }
    }

    // We can't implement a cheap, copy-less conversion because of utf-16, sigh
    pub fn to_string(&self) -> String {
        unsafe {
            (self.0 as usize != 0).then(|| {
                String::from_utf16(std::slice::from_raw_parts(
                    self.0.wrapping_add(20).cast::<u16>(),
                    self.len(),
                ))
                .unwrap_or("".to_string())
            }).unwrap_or("".to_string())
        }
    }
}
