fn encoder(b: u8) -> u8 {
    match b {
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
        _ => 0x0, 
    }
}

fn encode_hex(byte: u8) -> u8 {
    let high_nibble = encoder(byte >> 4); 
    let low_nibble = encoder(byte & 0x0F);    
    (high_nibble << 4) | low_nibble
}

fn encode_hex_data(v: &[u8]) -> Vec<u8> {
    let mut encoded_data = Vec::new();

    for &byte in v {
        encoded_data.push(encode_hex(byte));
    }

    encoded_data
}

#[test]
fn test_encode_hex() {
    const FOX: &str = "The quick brown fox jumps over the lazy dog.";
    const ENCODED_DATA: &[u8] = &[
    0x76, 0x5C, 0x57, 0x30, 0x41, 0x47, 0x5D, 0x52,
    0x5E, 0x30, 0x53, 0x43, 0x58, 0x44, 0x59, 0x30,
    0x55, 0x58, 0x4C, 0x30, 0x5F, 0x47, 0x5B, 0x40,
    0x42, 0x30, 0x58, 0x45, 0x57, 0x43, 0x30, 0x46,
    0x5C, 0x57, 0x30, 0x5A, 0x51, 0x4F, 0x4D, 0x30,
    0x56, 0x58, 0x54, 0x39
    ]; 

    assert_eq!(
        (0..16).map(encode_hex).collect::<Vec<_>>(),
        [ 0x0, 0x1, 0x3, 0x2, 0x6, 0x7, 0x5, 0x4,
        0xC, 0xD, 0xF, 0xE, 0xA, 0xB, 0x9, 0x8 ]);

    assert_eq!(encode_hex(0x54), 0x76);
    assert_eq!(encode_hex(0x68), 0x5C);

    let original_data = FOX.as_bytes();
    let encoded_data = ENCODED_DATA;
    assert_eq!(encode_hex_data(original_data), encoded_data);
}

fn swap_nibbles(b: u8) -> u8{((b & 0x0F) << 4) | (b >> 4)}

fn main(){
    let x = 0x54;
    let y = swap_nibbles(x);
    println!("{:02X}", y)
}