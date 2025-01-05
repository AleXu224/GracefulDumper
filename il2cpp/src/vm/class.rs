use super::*;

#[repr(transparent)]
pub struct Il2cppClass(pub *const u8);

impl std::fmt::Debug for Il2cppClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.il2cpp_type().name())
    }
}

impl Il2cppClass {
    pub fn init(&self) {
        unsafe { il2cpp_vm_class_init(self.0) }
    }

    pub fn init_methods(&self) {
        unsafe { il2cpp_vm_class_init_methods(self.0) }
    }

    pub fn name(&self) -> Cow<'static, str> {
        unsafe { cstr(il2cpp_class_get_name(self.0)) }
    }

    pub fn namespace(&self) -> Cow<'static, str> {
        unsafe { cstr(il2cpp_class_get_namespace(self.0)) }
    }

    pub fn image(&self) -> Il2cppImage {
        unsafe { Il2cppImage::from(il2cpp_class_get_image(self.0)) }
    }

    pub fn parent_class(&self) -> Option<Self> {
        let ptr = unsafe { il2cpp_class_get_parent(self.0) };
        ((ptr as usize) != 0).then_some(Self(ptr))
    }

    pub fn get_generic_argument(&self, index: usize) -> Il2cppType {
        unsafe { Il2cppType(il2cpp_class_get_generic_arg_type(self.0, index).cast()) }
    }

    pub fn get_generic_argument_count(&self) -> usize {
        unsafe { il2cpp_class_get_generic_arg_count(self.0) }
    }

    pub fn interfaces(&self) -> &[Il2cppClass] {
        unsafe {
            let mut count = 0;
            let interfaces = il2cpp_class_get_interfaces(self.0, &mut count);
            (count != 0)
                .then_some(std::slice::from_raw_parts(
                    mem::transmute(interfaces),
                    count,
                ))
                .unwrap_or_default()
        }
    }

    pub fn fields(&self) -> &[Il2cppField] {
        self.init();

        unsafe {
            let mut count = 0;
            let fields = il2cpp_class_get_fields(self.0, &mut count);
            std::slice::from_raw_parts(mem::transmute(fields), count)
        }
    }

    pub fn methods(&self) -> &[Il2cppMethod] {
        self.init_methods();

        unsafe {
            let mut count = 0;
            let methods = il2cpp_class_get_methods(self.0, &mut count);
            std::slice::from_raw_parts(mem::transmute(methods), count)
        }
    }

    pub fn find_method(&self, name: &str, arg_types: &[&str]) -> Option<Il2cppMethod> {
        self.init();
        self.init_methods();

        for method in self.methods() {
            if method.name() == name {
                if method.arg_count() == arg_types.len() {
                    let mut fail = false;
                    for i in 0..method.arg_count() {
                        if arg_types[i] != method.arg_type(i).name() {
                            fail = true;
                            break;
                        }
                    }

                    if !fail {
                        return Some(Il2cppMethod(method.0));
                    }
                }
            }
        }

        None
    }

    pub fn get_method(&self, name: &str, arg_count: i32) -> Option<Il2cppMethod> {
        self.init();
        self.init_methods();

        let ptr = unsafe {
            il2cpp_class_get_method_from_name(self.0, as_cstr!(name) as *const i8, arg_count)
        };

        (ptr as usize != 0).then_some(Il2cppMethod(ptr))
    }

    pub fn il2cpp_type(&self) -> Il2cppType {
        unsafe { Il2cppType(il2cpp_class_get_type(self.0)) }
    }

    pub fn token(&self) -> u32 {
        unsafe { il2cpp_class_get_token(self.0) }
    }
}
