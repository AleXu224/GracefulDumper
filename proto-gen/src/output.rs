use std::{borrow::Cow, fmt};

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
