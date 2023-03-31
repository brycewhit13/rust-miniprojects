// Functions
pub fn int_to_binary(int: u32) -> String {
    format!("{:b}", int)
}

pub fn int_to_hex(int: u32) -> String {
    format!("{:x}", int)
}

pub fn int_to_octal(int: u32) -> String {
    format!("{:o}", int)
}