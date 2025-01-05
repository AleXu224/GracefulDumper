// VarInt encoding helpers.
pub fn varint_length(mut v: u32) -> usize {
    if v == 0 {
        return 1;
    }

    let mut logcounter = 0;
    while v > 0 {
        logcounter += 1;
        v >>= 7;
    }
    logcounter
}

pub fn encode_varint(dst: &mut Vec<u8>, value: u32) -> usize {
    const MSB: u8 = 0b1000_0000;

    let mut n = value;
    let mut i = 0;

    while n >= 0x80 {
        dst.push(MSB | (n as u8));
        i += 1;
        n >>= 7;
    }

    dst.push(n as u8);
    i + 1
}

// Wire types. 3 and 4 are deprecated and hoyo don't use them (SGROUP and EGROUP)
pub const WIRE_TYPE_VAR_INT: u8 = 0;
pub const WIRE_TYPE_I64: u8 = 1;
pub const WIRE_TYPE_LENGTH_PREFIXED: u8 = 2;
pub const WIRE_TYPE_I32: u8 = 5;

#[inline]
pub fn pack_wire_tag(field_id: u32, wire_type: u8) -> u32 {
    (field_id << 3) | (wire_type as u32)
}
