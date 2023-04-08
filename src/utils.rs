pub fn path_exists(path: &str) -> bool {
    std::fs::metadata(path).is_ok()
}

pub fn char_to_int(c: char) -> u8 {
    match c {
        '0'..='9' => c as u8 - b'0',
        'A'..='F' => c as u8 - b'A' + 10,
        'a'..='f' => c as u8 - b'a' + 10,
        _ => 0,
    }
}

pub fn hex_to_bytes(input: &str, output: &mut [u8]) {
    let input_len = input.len();
    let mut i = 0;
    while i < input_len {
        output[i / 2] = char_to_int(input.chars().nth(i).unwrap()) * 16
            + char_to_int(input.chars().nth(i + 1).unwrap());
        i += 2;
    }
}