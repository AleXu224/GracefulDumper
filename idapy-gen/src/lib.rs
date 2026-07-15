mod util;

use std::io::{self, Write};

use il2cpp::{vm::Il2cppDomain};
use metadata::MetadataUsage;
use util::write_escaped_str;

// We don't want to overcomplicate this by using a full-fledged json library, so we're just building json by hand
pub unsafe fn write_to_file<W: Write>(out: &mut W) -> io::Result<()> {
    write!(out, "{{")?;

    write!(out, "\"ScriptString\": ")?;
    write_string_literals(out)?;
    write!(out, ", \"ScriptMetadata\": ")?;
    write_type_info(out)?;
    write!(out, ", \"ScriptMethod\": ")?;
    write_methods(out)?;
    write!(out, ", \"ScriptMetadataMethod\": ")?;
    write_method_refs(out)?;

    write!(out, "}}")
}

unsafe fn write_methods<W: Write>(out: &mut W) -> io::Result<()> {
    write!(out, "[")?;

    let mut first_write = true;
    for assembly in Il2cppDomain::get().assemblies() {
        let img = assembly.image();
        for i in 0..img.get_class_count() {
            let class = img.get_class(i);
            class.init();
            class.init_methods();

            for method in class.methods() {
                if !first_write {
                    write!(out, ", ")?;
                }
                first_write = false;

                write!(
                    out,
                    "{{\"Address\":{}, \"Name\": \"{}$${}\", \"Signature\": \"\", \"TypeSignature\": \"\"}}",
                    method.address() - il2cpp::ffi::base(),
                    class.il2cpp_type().name(),
                    method.name(),
                )?;
            }
        }
    }

    write!(out, "]")
}

unsafe fn write_method_refs<W: Write>(out: &mut W) -> io::Result<()> {
    write!(out, "[")?;

    let mut first_write = true;
    for i in 0..metadata::USAGES_COUNT {
        if let Some(entry) = metadata::get_usage_by_index(i) {
            if let MetadataUsage::MethodRef(method) = entry.usage {
                if !first_write {
                    write!(out, ", ")?;
                }
                first_write = false;
                let type_name = method.class().il2cpp_type().name();
                write!(
                    out,
                    "{{\"Address\":{}, \"Name\": \"{}_{}\", \"MethodAddress\": {}}}",
                    entry.address - il2cpp::ffi::base(),
                    &type_name,
                    &method.name(),
                    method.address() - il2cpp::ffi::base(),
                )?;
            }
        }
    }

    write!(out, "]")
}

unsafe fn write_type_info<W: Write>(out: &mut W) -> io::Result<()> {
    write!(out, "[")?;

    let mut first_write = true;
    for i in 0..metadata::USAGES_COUNT {
        if let Some(entry) = metadata::get_usage_by_index(i) {
            if let MetadataUsage::TypeInfo(class) = entry.usage {
                if !first_write {
                    write!(out, ", ")?;
                }
                first_write = false;
                let type_name = class.il2cpp_type().name();
                write!(
                    out,
                    "{{\"Address\":{}, \"Name\": \"{}_TypeInfo\", \"Signature\": \"{}_c*\"}}",
                    entry.address - il2cpp::ffi::base(),
                    &type_name,
                    &type_name
                )?;
            }
        }
    }

    write!(out, "]")
}

unsafe fn write_string_literals<W: Write>(out: &mut W) -> io::Result<()> {
    write!(out, "[")?;

    let mut first_write = true;
    for i in 0..metadata::USAGES_COUNT {
        if let Some(entry) = metadata::get_usage_by_index(i) {
            if let MetadataUsage::StringLiteral(s) = entry.usage {
                let ss = s.to_string();
                if ss.is_empty() {
                    continue;
                }
                if !first_write {
                    write!(out, ", ")?;
                }
                first_write = false;

                write!(
                    out,
                    "{{\"Address\":{},\"Value\":\"",
                    entry.address - il2cpp::ffi::base()
                )?;
                write_escaped_str(out, &ss)?;
                write!(out, "\"}}")?;
            }
        }
    }

    write!(out, "]")
}
