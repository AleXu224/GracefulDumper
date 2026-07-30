use std::{borrow::Cow, fmt, io::{self, Write}};

pub struct ProtoFile {
    pub syntax: String,
    pub imports: Vec<String>,
    pub items: Vec<ProtoItem>,
}

pub struct Message {
    pub cmd_id: u16,
    pub name: String,
    pub fields: Vec<Field>,
    pub oneofs: Vec<Oneof>,
}

pub struct Enum {
    pub name: String,
    pub variants: Vec<(String, i32)>,
}

pub struct Field {
    pub kind: Cow<'static, str>,
    pub name: String,
    pub number: u32,
    pub comment: Option<FieldComment>,
    pub is_enum: bool,
}

pub struct FieldComment {
    pub offset: u32,
    pub xor_const: u32,
}

pub struct Oneof {
    pub name: String,
    pub fields: Vec<Field>,
}

pub enum ProtoItem {
    Message(Message),
    Enum(Enum),
}

impl ProtoFile {
    pub fn write_json<W: Write>(&self, out: &mut W) -> io::Result<()> {
        write!(out, "[")?;
        let mut first_message = true;
        for item in self.items.iter() {
            if let ProtoItem::Message(message) = item {
                if !first_message {
                    write!(out, ",")?;
                }
                first_message = false;
                write!(out, "{{\"name\":\"")?;
                write_escaped(out, &message.name)?;
                write!(out, "\",\"cmd_id\":")?;
                if message.cmd_id == 0 {
                    write!(out, "null")?;
                } else {
                    write!(out, "{}", message.cmd_id)?;
                }
                write!(out, ",\"fields\":[")?;

                let mut all_fields: Vec<&Field> = message.fields.iter().collect();
                for oneof in &message.oneofs {
                    all_fields.extend(oneof.fields.iter());
                }

                let mut first_field = true;
                for field in &all_fields {
                    if !first_field {
                        write!(out, ",")?;
                    }
                    first_field = false;

                    let (base_type, is_repeated) = strip_repeated(&field.kind);

                    write!(out, "{{\"number\":{},\"name\":\"", field.number)?;
                    write_escaped(out, &field.name)?;
                    write!(out, "\",\"type\":\"")?;
                    write_escaped(out, base_type)?;
                    write!(out, "\",\"repeated\":{}", is_repeated)?;
                    write!(out, ",\"is_native_type\":{}", is_native_type(base_type))?;
                    write!(out, ",\"is_enum\":{}", field.is_enum)?;
                    write!(out, ",\"xor_value\":")?;
                    match field.comment {
                        Some(ref c) => write!(out, "{}", c.xor_const)?,
                        None => write!(out, "null")?,
                    }
                    write!(out, "}}")?;
                }

                write!(out, "]}}")?;
            }
        }
        write!(out, "]")
    }
}

fn write_escaped<W: Write>(out: &mut W, s: &str) -> io::Result<()> {
    for &b in s.as_bytes() {
        match b {
            b'"' => write!(out, "\\\"")?,
            b'\\' => write!(out, "\\\\")?,
            b'\n' => write!(out, "\\n")?,
            b'\r' => write!(out, "\\r")?,
            b'\t' => write!(out, "\\t")?,
            b if b < 0x20 => write!(out, "\\u{:04x}", b)?,
            _ => out.write_all(&[b])?,
        }
    }
    Ok(())
}

fn is_native_type(kind: &str) -> bool {
    matches!(
        kind,
        "bool"
            | "int32"
            | "uint32"
            | "int64"
            | "uint64"
            | "float"
            | "double"
            | "string"
            | "bytes"
            | "google.protobuf.Any"
    )
}

fn strip_repeated(kind: &str) -> (&str, bool) {
    if let Some(base) = kind.strip_prefix("repeated ") {
        (base, true)
    } else {
        (kind, false)
    }
}

impl fmt::Display for ProtoFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "syntax = \"{}\";", self.syntax)?;
        for import in self.imports.iter() {
            writeln!(f, "import \"{}\";", import)?;
        }
        writeln!(f)?;

        for item in self.items.iter() {
            match item {
                ProtoItem::Message(message) => write!(f, "{message}")?,
                ProtoItem::Enum(enumeration) => write!(f, "{enumeration}")?,
            }
        }

        Ok(())
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "message {} {{", self.name)?;
        if self.cmd_id != 0 {
            write!(f, " // CmdID: {}", self.cmd_id)?;
        }
        writeln!(f)?;

        for field in self.fields.iter() {
            write!(f, "  {} {} = {};", field.kind, field.name, field.number)?;
            if let Some(comment) = field.comment.as_ref() {
                write!(
                    f,
                    " // offset: {}, xor const: {}",
                    comment.offset, comment.xor_const
                )?;
            }
            writeln!(f)?;
        }

        for oneof in self.oneofs.iter() {
            write!(f, "{oneof}")?;
        }

        writeln!(f, "}}\n")
    }
}

impl fmt::Display for Oneof {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  oneof {} {{", self.name)?;
        for field in self.fields.iter() {
            writeln!(f, "    {} {} = {};", field.kind, field.name, field.number)?;
        }
        writeln!(f, "  }}")
    }
}

impl fmt::Display for Enum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "enum {} {{", self.name)?;
        for (name, discriminant) in self.variants.iter() {
            writeln!(f, "  {name} = {discriminant};")?;
        }
        writeln!(f, "}}\n")
    }
}
