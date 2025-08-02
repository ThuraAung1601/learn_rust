fn encode_hex(byte: u8) -> u8 {
    match byte {
        0x0 => 0x0,
        0x1 => 0x1,
        0x2 => 0x3,
        0x3 => 0x2,
        0x4 => 0x6,
        0x5 => 0x7,
        0x6 => 0x5,
        0x7 => 0x4,
        0x8 => 0xC,
        0x9 => 0xD,
        0xA => 0xF,
        0xB => 0xE,
        0xC => 0xA,
        0xD => 0xB,
        0xE => 0x9,
        0xF => 0x8,
        _ => panic!("Invalid input: Byte value must be between 0x0 and 0xF."),
    }
}

fn encode_hex_data(data: &[u8]) -> Vec<u8> {
    let mut encoded_data = Vec::with_capacity(data.len());

    for &byte in data {
        let encoded_byte = encode_hex(byte);
        encoded_data.push(encoded_byte);
    }

    encoded_data
}

fn u8_to_binary(byte_value: u8) -> String {
    let mut binary = String::new();
    let mut value = byte_value;

    for _ in 0..8 {
        let remainder = value % 2;
        binary.insert(0, std::char::from_digit(remainder as u32, 10).unwrap());
        value /= 2;
    }

    binary
}

fn main() { 
    let input_string = "Hello".as_bytes(); // Replace this with your input string.
    let encoded_result = encode_hex_data(input_string);

    println!("Encoded Data: {:?}", encoded_result);
}


