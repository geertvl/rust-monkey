pub fn convert_u32(val: impl TryInto<u32>) -> u32 {
    let val = match val.try_into() {
        Ok(v) => v,
        Err(_) => panic!("cannot convert to u32"),
    };
    val
}

pub fn convert_usize(val: impl TryInto<usize>) -> usize {
    match val.try_into() {
        Ok(v) => v,
        Err(_) => panic!("cannot convert to usize"),
    }
}

pub fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

pub fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

