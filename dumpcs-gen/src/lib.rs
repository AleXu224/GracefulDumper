use il2cpp::vm::{attributes::*, Il2cppField, Il2cppMethod};
use std::io::{self, Write};

use il2cpp::vm::{Il2cppDomain, Il2cppString};

unsafe fn write_assemblies_list<W: Write>(out: &mut W, domain: &Il2cppDomain) -> io::Result<()> {
    for assembly in domain.assemblies() {
        let image = assembly.image();

        writeln!(
            out,
            "// Assembly {}, class count: {}",
            image.name(),
            image.get_class_count()
        )?;
    }

    writeln!(out)
}

pub unsafe fn dump<W: Write>(out: &mut W) -> io::Result<()> {
    il2cpp::ffi::il2cpp_gc_disable();
    let domain = Il2cppDomain::get();

    // First write assemblies list on top of dump.cs
    write_assemblies_list(out, &domain)?;

    for assembly in domain.assemblies() {
        let image = assembly.image();
        let image_name = image.name();

        for i in 0..image.get_class_count() {
            let class = image.get_class(i);
            class.init();
            class.init_methods();

            let namespace = class.namespace();

            writeln!(out, "// Assembly: {image_name}, Namespace: {namespace}")?;
            write!(out, "class {}", class.il2cpp_type().name())?;

            if let Some(parent) = class.parent_class() {
                write!(out, " : {}", parent.il2cpp_type().name())?;
            }

            for (i, interface) in class.interfaces().into_iter().enumerate() {
                if i == 0 && class.parent_class().is_none() {
                    write!(out, " : {}", interface.il2cpp_type().name())?;
                } else {
                    write!(out, ", {}", interface.il2cpp_type().name())?;
                }
            }

            writeln!(out, " {{ // Token: 0x{:X}", class.token())?;

            class
                .fields()
                .iter()
                .try_for_each(|field| write_class_field(out, field))?;

            writeln!(out)?;

            class
                .methods()
                .iter()
                .try_for_each(|method| write_class_method(out, method))?;

            writeln!(out, "}}\n")?;
        }
    }

    Ok(())
}

fn write_class_field<W: Write>(out: &mut W, field: &Il2cppField) -> io::Result<()> {
    let field_type = field.il2cpp_type();
    let attrs = field_type.attrs();

    write!(out, "    ")?;
    prepend_field_modifiers(out, attrs)?;
    write!(out, "{} {}", field_type.name(), field.name())?;

    if attrs & FIELD_ATTRIBUTE_LITERAL != 0 {
        let value = field.static_get_value();

        match field_type.type_enum() {
            0x3 => write!(out, " = '{}'", char::from(value as u8))?,
            0xC => write!(out, " = {}", f32::from_bits(value as u32))?,
            0xD => write!(out, " = {}", f64::from_bits(value as u64))?,
            0xE => write!(
                out,
                " = \"{}\"",
                Il2cppString(value as *const u8).to_string()
            )?,
            _ => write!(out, " = {value}")?,
        }
    }

    writeln!(
        out,
        "; // Offset: 0x{:X}, Token: 0x{:X}",
        field.offset(),
        field.token()
    )
}

fn write_class_method<W: Write>(out: &mut W, method: &Il2cppMethod) -> io::Result<()> {
    write!(
        out,
        "    // RVA: 0x{:X}\n    ",
        (method.address() != 0)
            .then_some(method.address() - il2cpp::ffi::base())
            .unwrap_or(0)
    )?;

    prepend_method_modifiers(out, method.attrs())?;
    write!(out, "{} {}(", method.return_type().name(), method.name())?;

    for i in 0..method.arg_count() {
        if i != 0 {
            write!(out, ", ")?;
        }
        write!(out, "{} {}", method.arg_type(i).name(), method.arg_name(i))?;
    }

    writeln!(out, ") {{}}\n")
}

fn prepend_field_modifiers<W: Write>(out: &mut W, attrs: u32) -> io::Result<()> {
    match attrs & FIELD_ATTRIBUTE_FIELD_ACCESS_MASK {
        FIELD_ATTRIBUTE_PUBLIC => write!(out, "public "),
        FIELD_ATTRIBUTE_PRIVATE => write!(out, "private "),
        FIELD_ATTRIBUTE_FAMILY => write!(out, "protected "),
        FIELD_ATTRIBUTE_ASSEMBLY | FIELD_ATTRIBUTE_FAM_AND_ASSEM => {
            write!(out, "internal ")
        }
        FIELD_ATTRIBUTE_FAM_OR_ASSEM => write!(out, "protected internal "),
        _ => Ok(()),
    }?;

    if attrs & FIELD_ATTRIBUTE_LITERAL != 0 {
        write!(out, "const ")?;
    } else if attrs & FIELD_ATTRIBUTE_STATIC != 0 {
        write!(out, "static ")?;
    }

    if attrs & FIELD_ATTRIBUTE_INIT_ONLY != 0 {
        write!(out, "readonly ")?;
    }

    Ok(())
}

fn prepend_method_modifiers<W: Write>(out: &mut W, attrs: u32) -> io::Result<()> {
    match attrs & METHOD_ATTRIBUTE_MEMBER_ACCESS_MASK {
        METHOD_ATTRIBUTE_PUBLIC => write!(out, "public "),
        METHOD_ATTRIBUTE_PRIVATE => write!(out, "private "),
        METHOD_ATTRIBUTE_FAMILY => write!(out, "protected "),
        METHOD_ATTRIBUTE_ASSEM | METHOD_ATTRIBUTE_FAM_AND_ASSEM => {
            write!(out, "internal ")
        }
        METHOD_ATTRIBUTE_FAM_OR_ASSEM => write!(out, "protected internal "),
        _ => Ok(()),
    }?;

    if attrs & METHOD_ATTRIBUTE_STATIC != 0 {
        write!(out, "static ")?;
    }
    if attrs & METHOD_ATTRIBUTE_ABSTRACT != 0 {
        write!(out, "abstract ")?;
        if attrs & METHOD_ATTRIBUTE_VTABLE_LAYOUT_MASK == METHOD_ATTRIBUTE_REUSE_SLOT {
            write!(out, "override ")?;
        }
    } else if attrs & METHOD_ATTRIBUTE_FINAL != 0 {
        if attrs & METHOD_ATTRIBUTE_VTABLE_LAYOUT_MASK == METHOD_ATTRIBUTE_REUSE_SLOT {
            write!(out, "sealed override ")?;
        }
    } else if attrs & METHOD_ATTRIBUTE_VIRTUAL != 0 {
        if attrs & METHOD_ATTRIBUTE_VTABLE_LAYOUT_MASK == METHOD_ATTRIBUTE_NEW_SLOT {
            write!(out, "virtual ")?;
        } else {
            write!(out, "override ")?;
        }
    }
    if attrs & METHOD_ATTRIBUTE_PINVOKE_IMPL != 0 {
        write!(out, "extern ")?;
    }

    Ok(())
}
