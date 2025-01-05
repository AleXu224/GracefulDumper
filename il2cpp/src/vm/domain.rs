use super::*;

#[repr(transparent)]
pub struct Il2cppDomain(usize);

impl Il2cppDomain {
    pub fn get() -> Self {
        unsafe { Il2cppDomain(il2cpp_domain_get()) }
    }

    pub fn attach_thread(&self) {
        unsafe {
            il2cpp_thread_attach(self.0 as *const u8);
        }
    }

    pub fn assembly_open(&self, name: &str) -> Assembly {
        unsafe { Assembly::from(il2cpp_domain_assembly_open(self.0, as_cstr!(name)) as *const u8) }
    }

    pub fn assemblies(&self) -> &[Assembly] {
        unsafe {
            let mut count = 0;
            let assemblies = il2cpp_domain_get_assemblies(self.0, &mut count);

            std::slice::from_raw_parts(mem::transmute(assemblies), count)
        }
    }
}
