use super::*;

#[repr(transparent)]
pub struct Il2cppMethod(pub *const u8);

impl Il2cppMethod {
    pub fn name(&self) -> Cow<'static, str> {
        unsafe { cstr(il2cpp_method_get_name(self.0)) }
    }

    pub fn address(&self) -> usize {
        unsafe { il2cpp_method_get_address(self.0) }
    }

    pub fn arg_count(&self) -> usize {
        unsafe { il2cpp_method_get_arg_count(self.0) }
    }

    pub fn arg_name(&self, index: usize) -> Cow<'static, str> {
        unsafe { cstr(il2cpp_method_get_arg_name(self.0, index)) }
    }

    pub fn arg_type(&self, index: usize) -> Il2cppType {
        unsafe { Il2cppType(il2cpp_method_get_arg_type(self.0, index)) }
    }

    pub fn return_type(&self) -> Il2cppType {
        unsafe { Il2cppType(il2cpp_method_get_return_type(self.0).cast()) }
    }

    pub fn attrs(&self) -> u32 {
        unsafe { il2cpp_method_get_attrs(self.0) }
    }

    pub fn class(&self) -> Il2cppClass {
        unsafe { Il2cppClass(il2cpp_method_get_class(self.0)) }
    }

    pub fn invoke<T: From<usize>>(
        &self,
        instance: &dyn Il2cppValue,
        args: &[&dyn Il2cppValue],
    ) -> Result<T, Il2cppException> {
        let args = args.iter().map(|arg| arg.as_raw()).collect::<Vec<_>>();

        let mut exception = 0;
        let ret = unsafe {
            il2cpp_runtime_invoke(
                self.0,
                instance.as_raw() as *const u8,
                args.as_ptr() as *const usize,
                &mut exception,
            )
        };

        (exception == 0)
            .then_some(T::from(ret))
            .ok_or(Il2cppException::from(Il2cppObject::from(
                exception as *const u8,
            )))
    }
}
