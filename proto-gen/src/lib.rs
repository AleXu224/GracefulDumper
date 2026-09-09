use std::{
    borrow::Cow,
    collections::BTreeMap,
    io,
};

use cache::{CachedType, TypeCache};
use il2cpp::{vm::*};
use output::{Enum, Field, FieldComment, Message, Oneof, ProtoFile};
use util::{
    pack_wire_tag, WIRE_TYPE_I32, WIRE_TYPE_I64, WIRE_TYPE_LENGTH_PREFIXED, WIRE_TYPE_VAR_INT,
};

mod cache;
mod output;
mod util;

// Names
const CODED_INPUT_STREAM: &str = "CPAGGPCFFMK";
const MERGE_FROM: &str = "DFPJONEONIO";
const GET_CMD_ID: &str = "LCKPHIEEODG";
const UNKNOWN_FIELD_SET: &str = "HCFKHMCLBHN";
const BYTE_STRING: &str = "ODLKAIJKIPI";
const PROTOBUF_ANY: &str = "JCCKNPJGJIN";

struct TrackedValues<'tc> {
    type_cache: &'tc TypeCache,
    pub values: Vec<(u32, usize)>,
}

pub struct MessageMinimalInfo {
    pub cmd_id: u16,
    pub fields: Vec<FieldMinimalInfo>,
}

impl MessageMinimalInfo {
    pub fn new(cmd_id: u16) -> Self {
        Self {
            cmd_id,
            fields: Vec::new(),
        }
    }
}

pub struct FieldDetectionInfo {
    pub value: u32,
    pub offset: u32,
    pub oneof_extra_data: Option<OneofVariantInfo>,
}

pub struct FieldMinimalInfo {
    pub tag: u32,
    pub xor: u32,
    pub offset: u32,
    pub oneof_extra_data: Option<OneofVariantInfo>,
}

pub struct OneofVariantInfo {
    pub oneof_enum_offset: u32,
    pub variant_type: Il2cppType,
}

impl<'tc> TrackedValues<'tc> {
    pub fn new(tc: &'tc TypeCache, object: &Il2cppObject) -> Self {
        // dump initial values
        let mut values = Vec::with_capacity(object.class().fields().len());
        for field in object.class().fields() {
            if field.is_instance() && field.il2cpp_type().name() != UNKNOWN_FIELD_SET {
                values.push((field.offset(), Self::fetch_value(tc, field, object)));
            }
        }

        Self {
            type_cache: tc,
            values,
        }
    }

    // We need this because RepeatedField and Map need special handling
    fn fetch_value(cache: &TypeCache, field: &Il2cppField, object: &Il2cppObject) -> usize {
        let field_class = field.il2cpp_type().to_class();
        if let Some(ty) = cache.type_map.get(&(field_class.0 as usize)) {
            use CachedType::*;
            unsafe {
                match ty {
                    Boolean | Byte | SByte => *field.get_field_data_ptr(object) as usize,
                    Int16 | UInt16 => *field.get_field_data_ptr(object).cast::<u16>() as usize,
                    Single | Int32 | UInt32 => {
                        *field.get_field_data_ptr(object).cast::<u32>() as usize
                    }
                    Double | Int64 | UInt64 => {
                        *field.get_field_data_ptr(object).cast::<u64>() as usize
                    }
                    Object | Any | ByteString | String => {
                        *field.get_field_data_ptr(object).cast::<usize>()
                    }
                    _ => unreachable!(),
                }
            }
        } else {
            if field_class.get_generic_argument_count() != 0 {
                let collection = unsafe {
                    Il2cppObject(*(field.get_field_data_ptr(object).cast::<usize>()) as *const u8)
                };
                let get_count = collection.class().find_method("get_Count", &[]).unwrap();

                // method returns an object-packed (boxed) value
                unsafe {
                    *(get_count
                        .invoke::<Il2cppObject>(&collection, &[])
                        .unwrap()
                        .0
                        .wrapping_add(16)
                        .cast::<i32>()) as usize
                }
            } else if field_class
                .parent_class()
                .map(|p| matches!(cache.type_map.get(&(p.0 as usize)), Some(&CachedType::Enum)))
                .unwrap_or(false)
            {
                unsafe { *field.get_field_data_ptr(object).cast::<u32>() as usize }
            } else {
                unsafe { *field.get_field_data_ptr(object).cast::<usize>() }
            }
        }
    }

    pub fn detect_changes_and_update(&mut self, object: &Il2cppObject) -> Vec<u32> {
        let mut changed_offsets = Vec::new();
        for field in object.class().fields() {
            if field.is_instance() && field.il2cpp_type().name() != UNKNOWN_FIELD_SET {
                let value = Self::fetch_value(&self.type_cache, field, object);
                if value != self.get_saved_value(field.offset()) {
                    changed_offsets.push(field.offset());
                    self.replace_value(field.offset(), value);
                }
            }
        }

        changed_offsets
    }

    pub fn get_saved_value(&self, offset: u32) -> usize {
        self.values
            .iter()
            .find(|(off, _)| offset == *off)
            .unwrap()
            .1
    }

    pub fn replace_value(&mut self, offset: u32, value: usize) {
        self.values
            .iter_mut()
            .find(|(off, _)| offset == *off)
            .unwrap()
            .1 = value;
    }
}

unsafe fn build_proto_file() -> io::Result<ProtoFile> {
    il2cpp::ffi::il2cpp_gc_disable();
    let domain = Il2cppDomain::get();
    domain.attach_thread();
    let type_cache = TypeCache::init(&domain);
    let nap_proto_gen = domain.assembly_open("NapProtoGen.dll").image();

    let mut minimal_info_map = BTreeMap::new();
    for i in 0..nap_proto_gen.get_class_count() {
        let proto_class = nap_proto_gen.get_class(i);
        let Some(get_cmd_id) = proto_class.get_method(GET_CMD_ID, 0) else {
            continue;
        };

        let proto_instance = Il2cppObject::new(&proto_class);
        proto_class
            .get_method(".ctor", 0)
            .unwrap()
            .invoke::<usize>(&proto_instance, &[])
            .unwrap();

        let cmd_id_result = get_cmd_id
            .invoke::<Il2cppObject>(&proto_instance, &[])
            .unwrap();
        if cmd_id_result.0.is_null() {
            panic!("GetCmdId returned null");
        }
        let cmd_id = *(cmd_id_result.0.wrapping_add(16) as *const u16);

        let mut message_info = MessageMinimalInfo::new(cmd_id);

        let mut bruteforcer = Bruteforcer::new(&type_cache, proto_instance);

        // Pre-estimate number of fields for messages without oneof
        let estimated_field_count = (!proto_class
            .fields()
            .iter()
            .any(|f| f.il2cpp_type().name() == "System.Object"))
        .then_some(bruteforcer.tracked_values.values.len());

        for field_id in 1..4096 {
            if let Some(count) = estimated_field_count {
                if message_info.fields.len() >= count {
                    break;
                }
            }

            // First: try as varint
            let tag = pack_wire_tag(field_id, WIRE_TYPE_VAR_INT);
            if let Ok(Some(info)) = bruteforcer.input(tag, &[1]) {
                // varint fields are likely to get xored, so we're using varint of value 1 as input
                // and xoring set value with it, to get the xor constant
                message_info.fields.push(FieldMinimalInfo {
                    xor: info.value ^ 1,
                    offset: info.offset,
                    tag,
                    oneof_extra_data: info.oneof_extra_data,
                });
                continue;
            }

            // An empty length-prefixed data
            // This will work for nested messages. Non-null but all fields are default.
            // For collections this won't change anything

            let tag = pack_wire_tag(field_id, WIRE_TYPE_LENGTH_PREFIXED);
            if let Ok(Some(info)) = bruteforcer.input(tag, &[0]) {
                message_info.fields.push(FieldMinimalInfo {
                    xor: 0,
                    offset: info.offset,
                    tag,
                    oneof_extra_data: info.oneof_extra_data,
                });
                continue;
            }

            // Now let's try to bruteforce collections
            match bruteforcer.input(tag, &[1, 0]) {
                Ok(Some(info)) => {
                    // Length: 1, value: 0
                    // OR RepeatedField Count: 1 and empty message!
                    // OR ByteString with 1 byte (0)
                    // OR System.String '\0'

                    message_info.fields.push(FieldMinimalInfo {
                        xor: 0,
                        offset: info.offset,
                        tag,
                        oneof_extra_data: info.oneof_extra_data,
                    });
                }
                Err(_exception) => {
                    // Got an exception!
                    // This means that field with this tag definitely exists
                    // but we don't know which one yet, because nothing is set yet!
                    // reason of exception: data format is incorrect (maybe expected a map)
                    // or length was not enough (for fixed32/float/etc for example...)

                    const LENGTH_PREFIXED_SAMPLES: &[&[u8]] = &[
                        &[1, 0, 0, 0, 0],                   // repeated fixed32/float
                        &[1, 0, 0, 0, 0, 0, 0, 0, 0],       // repeated fixed64/double
                        &[5, 0x08, 0x01, 0x33, 0x01, 0x00], // map<varint, varint/string>
                        &[5, 0x10, 0x01, 0x33, 0x10, 0x00], // map<string, varint/string>
                    ];

                    for sample in LENGTH_PREFIXED_SAMPLES.iter() {
                        match bruteforcer.input(tag, sample) {
                            Ok(Some(info)) => {
                                message_info.fields.push(FieldMinimalInfo {
                                    xor: 0,
                                    offset: info.offset,
                                    tag,
                                    oneof_extra_data: info.oneof_extra_data,
                                });
                                break;
                            }
                            Err(_exc) => (), // try another sample!
                            Ok(None) => unreachable!(),
                        }
                    }
                }
                Ok(None) => (), // the specified tag doesn't exist in this message
            }

            // fixed32/float

            let tag = pack_wire_tag(field_id, WIRE_TYPE_I32);
            if let Ok(Some(info)) = bruteforcer.input(tag, &1u32.to_be_bytes()) {
                message_info.fields.push(FieldMinimalInfo {
                    xor: 0,
                    offset: info.offset,
                    tag,
                    oneof_extra_data: info.oneof_extra_data,
                });
            }

            // fixed64/double

            let tag = pack_wire_tag(field_id, WIRE_TYPE_I64);
            if let Ok(Some(info)) = bruteforcer.input(tag, &1u64.to_be_bytes()) {
                message_info.fields.push(FieldMinimalInfo {
                    xor: 0,
                    offset: info.offset,
                    tag,
                    oneof_extra_data: info.oneof_extra_data,
                });
            }
        }

        minimal_info_map.insert(proto_class.token(), message_info);
    }

    // Now build proto file from obtained MessageMinimalInfos

    let mut proto_file = ProtoFile {
        syntax: String::from("proto3"),
        imports: vec![String::from("google/protobuf/any.proto")],
        items: Vec::with_capacity(nap_proto_gen.get_class_count()),
    };

    for i in 0..nap_proto_gen.get_class_count() {
        let class = nap_proto_gen.get_class(i);
        if class.name() == "__HOLLOW__1_0" {
            continue;
        }
        if let Some(message_info) = minimal_info_map.get(&class.token()) {
            let mut fields = Vec::with_capacity(message_info.fields.len());

            for field_info in message_info
                .fields
                .iter()
                .filter(|f| f.oneof_extra_data.is_none())
            {
                let field = class
                    .fields()
                    .iter()
                    .find(|f| f.is_instance() && f.offset() == field_info.offset)
                    .unwrap();

                fields.push((
                    field.token(),
                    Field {
                        kind: csharp_type_to_protobuf_type(
                            &type_cache,
                            &field.il2cpp_type().to_class(),
                        ),
                        name: field.name().to_string(),
                        number: field_info.tag >> 3,
                        comment: Some(FieldComment {
                            offset: field.offset(),
                            xor_const: field_info.xor,
                        }),
                        is_enum: is_enum_type(&type_cache, &field.il2cpp_type().to_class()),
                    },
                ));
            }

            // We're sorting fields by their token
            // even though proto field ordering is shuffled before translation to C#
            // for the ones that aren't shuffled at all, correct field order is preserved
            fields.sort_by_key(|(token, _)| *token);
            let mut oneofs = Vec::<Oneof>::new();

            for field_info in message_info
                .fields
                .iter()
                .filter(|f| f.oneof_extra_data.is_some())
            {
                let oneof_data_field = class
                    .fields()
                    .iter()
                    .find(|f| f.is_instance() && f.offset() == field_info.offset)
                    .unwrap();
                let oneof_variant = field_info.oneof_extra_data.as_ref().unwrap();
                let oneof_enum_field = class
                    .fields()
                    .iter()
                    .find(|f| f.is_instance() && f.offset() == oneof_variant.oneof_enum_offset)
                    .unwrap();
                let oneof_enum = oneof_enum_field.il2cpp_type().to_class();
                let oneof_case_enum_field = oneof_enum
                    .fields()
                    .iter()
                    .find(|f| {
                        !f.is_instance() && f.static_get_value() as u32 == (field_info.tag >> 3)
                    })
                    .unwrap();

                let oneof = if let Some(oneof) = oneofs
                    .iter_mut()
                    .find(|o| o.name == oneof_data_field.name())
                {
                    oneof
                } else {
                    oneofs.push(Oneof {
                        name: oneof_data_field.name().to_string(),
                        fields: Vec::new(),
                    });
                    oneofs.last_mut().unwrap()
                };

                oneof.fields.push(Field {
                    kind: csharp_type_to_protobuf_type(
                        &type_cache,
                        &oneof_variant.variant_type.to_class(),
                    ),
                    name: oneof_case_enum_field.name().to_string(),
                    number: field_info.tag >> 3,
                    comment: None,
                    is_enum: is_enum_type(&type_cache, &oneof_variant.variant_type.to_class()),
                });
            }

            proto_file.items.push(output::ProtoItem::Message(Message {
                name: class.name().to_string(),
                cmd_id: message_info.cmd_id,
                fields: fields.into_iter().map(|f| f.1).collect(),
                oneofs,
            }));
        } else if class
            .parent_class()
            .map(|p| {
                matches!(
                    type_cache.type_map.get(&(p.0 as usize)),
                    Some(&CachedType::Enum)
                )
            })
            .unwrap_or(false)
        {
            let enum_name = class.name().to_string();
            proto_file.items.push(output::ProtoItem::Enum(Enum {
                variants: class
                    .fields()
                    .iter()
                    .filter(|f| !f.is_instance() && f.static_get_value() == 0)
                    .chain(
                        class
                            .fields()
                            .into_iter()
                            .filter(|f| !f.is_instance() && f.static_get_value() != 0),
                    )
                    .map(|f| {
                        (
                            // Google with their C++ism (enum scoping rules)
                            format!("{}_{}", &enum_name, f.name().to_string()),
                            f.static_get_value() as i32,
                        )
                    })
                    .collect(),
                name: enum_name,
            }));
        }
    }

    Ok(proto_file)
}

pub unsafe fn dump() -> io::Result<ProtoFile> {
    build_proto_file()
}

fn csharp_type_to_protobuf_type(cache: &TypeCache, ty: &Il2cppClass) -> Cow<'static, str> {
    if let Some(ty) = cache.type_map.get(&(ty.0 as usize)) {
        use CachedType::*;
        match ty {
            Boolean => Cow::Borrowed("bool"),
            Int32 => Cow::Borrowed("int32"),
            UInt32 => Cow::Borrowed("uint32"),
            Int64 => Cow::Borrowed("int64"),
            UInt64 => Cow::Borrowed("uint64"),
            Single => Cow::Borrowed("float"),
            Double => Cow::Borrowed("double"),
            String => Cow::Borrowed("string"),
            ByteString => Cow::Borrowed("bytes"),
            Any => Cow::Borrowed("google.protobuf.Any"),
            _ => unreachable!(),
        }
    } else {
        match ty.get_generic_argument_count() {
            1 => Cow::Owned(format!(
                "repeated {}",
                csharp_type_to_protobuf_type(cache, &ty.get_generic_argument(0).to_class())
            )),
            2 => Cow::Owned(format!(
                "map<{}, {}>",
                csharp_type_to_protobuf_type(cache, &ty.get_generic_argument(0).to_class()),
                csharp_type_to_protobuf_type(cache, &ty.get_generic_argument(1).to_class())
            )),
            _ => ty.name(),
        }
    }
}

fn is_enum_type(cache: &TypeCache, ty: &Il2cppClass) -> bool {
    match ty.get_generic_argument_count() {
        0 => ty
            .parent_class()
            .map(|p| matches!(cache.type_map.get(&(p.0 as usize)), Some(&CachedType::Enum)))
            .unwrap_or(false),
        1 => is_enum_type(cache, &ty.get_generic_argument(0).to_class()),
        2 => is_enum_type(cache, &ty.get_generic_argument(1).to_class()),
        _ => false,
    }
}

struct Bruteforcer<'tc> {
    object: Il2cppObject,
    tracked_values: TrackedValues<'tc>,
    merge_from_method: Il2cppMethod,
}

impl<'tc> Bruteforcer<'tc> {
    pub fn new(type_cache: &'tc TypeCache, object: Il2cppObject) -> Self {
        let tracked_values = TrackedValues::new(type_cache, &object);
        let merge_from_method = object
            .class()
            .find_method(MERGE_FROM, &[CODED_INPUT_STREAM])
            .expect("failed to find MergeFrom(CodedInputStream)");

        Self {
            object,
            tracked_values,
            merge_from_method,
        }
    }

    pub unsafe fn input(
        &mut self,
        wire_tag: u32,
        data: &[u8],
    ) -> Result<Option<FieldDetectionInfo>, Il2cppException> {
        let mut buf = Vec::with_capacity(data.len() + util::varint_length(wire_tag));
        util::encode_varint(&mut buf, wire_tag);
        buf.extend(data);

        self.merge_from_method
            .invoke::<Void>(&self.object, &[&create_input_stream(&buf)])?;

        let changed_offsets = self.tracked_values.detect_changes_and_update(&self.object);
        match changed_offsets.as_slice() {
            &[offset] => Ok(Some(FieldDetectionInfo {
                value: self.tracked_values.get_saved_value(offset) as u32,
                offset,
                oneof_extra_data: None,
            })),
            &[first, second] => {
                // oneof (storage field + enum case)

                let class = self.object.class();
                let first_field = class.fields().iter().find(|f| f.offset() == first).unwrap();

                let (data_offset, oneof_enum_offset) =
                    if first_field.il2cpp_type().to_class().name() == "Object" {
                        (first, second)
                    } else {
                        (second, first)
                    };

                let data_field_type = Il2cppClass(
                    *(*(self.object.0.wrapping_add(data_offset as usize) as *const usize)
                        as *const usize) as *const u8,
                );

                Ok(Some(FieldDetectionInfo {
                    value: 0,
                    offset: data_offset,
                    oneof_extra_data: Some(OneofVariantInfo {
                        oneof_enum_offset,
                        variant_type: data_field_type.il2cpp_type(),
                    }),
                }))
            }
            &[] => Ok(None),
            _ => panic!(
                "abnormal number of fields changed: {}",
                changed_offsets.len()
            ),
        }
    }
}

pub unsafe fn create_input_stream(buf: &[u8]) -> Il2cppObject {
    let protobuf_lib = Il2cppDomain::get().assembly_open("Protobuf.dll").image();

    let coded_input_stream = Il2cppObject::new(
        &protobuf_lib
            .get_class_by_name("", CODED_INPUT_STREAM)
            .expect("failed to find CodedInputStream in Protobuf.dll"),
    );

    let constructor = coded_input_stream
        .class()
        .find_method(".ctor", &["System.Byte[]"])
        .expect("failed to find CodedInputStream.ctor(byte[])");

    let byte_array_class = constructor.arg_type(0).to_class();
    let byte_array = Il2cppArray::new(&byte_array_class, buf.len());
    byte_array.as_mut_slice().copy_from_slice(buf);

    constructor
        .invoke::<Il2cppObject>(&coded_input_stream, &[&byte_array])
        .unwrap();

    coded_input_stream
}
