#[allow(clippy::manual_range_contains)]
fn is_ascii_hexdigit_byte(b: u8) -> bool {
    (b >= b'0' && b <= b'9') || (b >= b'a' && b <= b'f') || (b >= b'A' && b <= b'F')
}
pub fn is_public_alias(name: &str) -> bool {
    let bytes = name.as_bytes();
    if bytes.len() != 19 {
        return false;
    }
    if bytes[0] != b'v' || bytes[1] != b'w' || bytes[2] != b'_' {
        return false;
    }
    #[allow(clippy::needless_range_loop)]
    for i in 3..19 {
        if !is_ascii_hexdigit_byte(bytes[i]) {
            return false;
        }
    }
    true
}
pub fn is_scratchpad_table(name: &str) -> bool {
    let bytes = name.as_bytes();
    if bytes.len() != 19 {
        return false;
    }
    if bytes[0] != b't' || bytes[1] != b'm' || bytes[2] != b'_' {
        return false;
    }
    #[allow(clippy::needless_range_loop)]
    for i in 3..19 {
        if !is_ascii_hexdigit_byte(bytes[i]) {
            return false;
        }
    }
    true
}
#[allow(clippy::manual_range_contains)]
pub fn eq_ignore_ascii_case(a: &str, b: &str) -> bool {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    if a_bytes.len() != b_bytes.len() {
        return false;
    }
    let n = a_bytes.len();
    for i in 0..n {
        let ca = a_bytes[i];
        let cb = b_bytes[i];
        let la = if ca >= b'A' && ca <= b'Z' {
            ca + 32
        } else {
            ca
        };
        let lb = if cb >= b'A' && cb <= b'Z' {
            cb + 32
        } else {
            cb
        };
        if la != lb {
            return false;
        }
    }
    true
}
#[allow(clippy::manual_range_contains)]
fn ascii_lower_u8(b: u8) -> u8 {
    if b >= b'A' && b <= b'Z' {
        b + 32
    } else {
        b
    }
}
fn is_range(s: &str) -> bool {
    let b = s.as_bytes();
    if b.len() != 5 {
        return false;
    }
    ascii_lower_u8(b[0]) == b'r'
        && ascii_lower_u8(b[1]) == b'a'
        && ascii_lower_u8(b[2]) == b'n'
        && ascii_lower_u8(b[3]) == b'g'
        && ascii_lower_u8(b[4]) == b'e'
}
fn is_main(s: &str) -> bool {
    let b = s.as_bytes();
    if b.len() != 4 {
        return false;
    }
    ascii_lower_u8(b[0]) == b'm'
        && ascii_lower_u8(b[1]) == b'a'
        && ascii_lower_u8(b[2]) == b'i'
        && ascii_lower_u8(b[3]) == b'n'
}
fn is_ai_scratchpad(s: &str) -> bool {
    let b = s.as_bytes();
    if b.len() != 13 {
        return false;
    }
    ascii_lower_u8(b[0]) == b'a'
        && ascii_lower_u8(b[1]) == b'i'
        && ascii_lower_u8(b[2]) == b'_'
        && ascii_lower_u8(b[3]) == b's'
        && ascii_lower_u8(b[4]) == b'c'
        && ascii_lower_u8(b[5]) == b'r'
        && ascii_lower_u8(b[6]) == b'a'
        && ascii_lower_u8(b[7]) == b't'
        && ascii_lower_u8(b[8]) == b'c'
        && ascii_lower_u8(b[9]) == b'h'
        && ascii_lower_u8(b[10]) == b'p'
        && ascii_lower_u8(b[11]) == b'a'
        && ascii_lower_u8(b[12]) == b'd'
}
pub fn is_valid_public_object_name(parts: &[String]) -> bool {
    if parts.len() == 1 {
        let object = &parts[0];
        if is_range(object) {
            return true;
        }
        if is_public_alias(object) {
            return true;
        }
        return false;
    }
    if parts.len() == 2 {
        let schema = &parts[0];
        let object = &parts[1];
        if is_public_alias(object) && (is_main(schema) || is_ai_scratchpad(schema)) {
            return true;
        }
        return false;
    }
    false
}
pub fn is_valid_write_target(parts: &[String]) -> bool {
    if parts.len() == 1 {
        return is_scratchpad_table(&parts[0]);
    }
    if parts.len() == 2 {
        return is_ai_scratchpad(&parts[0]) && is_scratchpad_table(&parts[1]);
    }
    false
}
