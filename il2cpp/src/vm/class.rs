use super::*;

#[repr(transparent)]
pub struct Il2cppClass(pub *const u8);
pub struct Il2CppTypeDefinition(pub *const u8);

impl std::fmt::Debug for Il2cppClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.il2cpp_type().name())
    }
}

impl Il2CppTypeDefinition {
    pub fn namespace_index(&self) -> u32 {
        unsafe { *(self.0.wrapping_add(12) as *const u32) }
    }
    pub fn namespace(&self) -> Cow<'static, str> {
        unsafe { cstr(metadatacache_getstringfromindex(self.namespace_index())) }
    }
    pub fn field_count(&self) -> u16 {
        unsafe { *(self.0.wrapping_add(60) as *const u16) }
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
        unsafe {
            let name_ptr = il2cpp_class_get_name(self.0);
            if name_ptr.is_null() {
                Cow::Borrowed("<null>")
            } else {
                cstr(name_ptr)
            }
        }
    }

    pub fn type_definition(&self) -> Il2CppTypeDefinition {
        unsafe { Il2CppTypeDefinition(*(self.0.wrapping_add(48) as *const usize) as *const u8) }
    }

    pub fn namespace(&self) -> Cow<'static, str> {
        self.type_definition().namespace()
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
            (count != 0 && (interfaces as usize != 0))
                .then(|| std::slice::from_raw_parts(mem::transmute(interfaces), count))
                .unwrap_or_default()
        }
    }

    pub fn fields(&self) -> &[Il2cppField] {
        self.init();

        unsafe {
            let count = self.type_definition().field_count() as usize;
            let fields = il2cpp_class_get_fields(self.0);
            (count != 0 && (fields as usize != 0))
                .then(|| std::slice::from_raw_parts(mem::transmute(fields), count))
                .unwrap_or_default()
        }
    }

    pub fn methods(&self) -> &[Il2cppMethod] {
        self.init_methods();

        unsafe {
            let mut count = 0;
            let methods = il2cpp_class_get_methods(self.0, &mut count);
            (count != 0 && (methods as usize != 0))
                .then(|| std::slice::from_raw_parts(mem::transmute(methods), count))
                .unwrap_or_default()
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
