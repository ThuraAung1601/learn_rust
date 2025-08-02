fn encode(b: u8) -> u8 {
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

fn decode(b: u8) -> u8 {
    match b {
        0x0 => 0x0,
        0x1 => 0x1,
        0x3 => 0x2,
        0x2 => 0x3,
        0x6 => 0x4,
        0x7 => 0x5,
        0x5 => 0x6,
        0x4 => 0x7,
        0xC => 0x8,
        0xD => 0x9,
        0xF => 0xA,
        0xE => 0xB,
        0xA => 0xC,
        0xB => 0xD,
        0x9 => 0xE,
        0x8 => 0xF,
        _ => 0x0,
    }
}

fn encoder(b: u8) -> u8 {
    let high_nibble = b >> 4;
    let low_nibble = b & 0xF;
    (encode(high_nibble) << 4) | encode(low_nibble) 
}

fn decoder(b: u8) -> u8 {
    let high_nibble = b >> 4;
    let low_nibble = b & 0xF;
    (decode(high_nibble) << 4) | decode(low_nibble)
}

fn swap_nibble(b: u8) -> u8 {
    let high_nibble = b >> 4;
    let low_nibble = b & 0xF;
    low_nibble << 4 | high_nibble
}

fn encoding(original_data: &[u8]) -> Vec<u8> {
    let mut encoded_data = Vec::new();
    for i in original_data {
        encoded_data.push(encoder(*i));
    }
    encoded_data
}

fn decoding(encoded_data: &[u8]) -> Vec<u8> {
    let mut decoded_data = Vec::new();
    for i in encoded_data {
        decoded_data.push(decoder(*i));
    }
    decoded_data
}

fn main() {
    let b = 0x54;
    println!("{}", b);
    println!("{:0x} => {:0x}", b, swap_nibble(b));
    println!("{:0x} => {:0x}", b, encoder(b));
    let encoded_b = encoder(b);
    println!("{:0x} => {:0x}", encoded_b, decoder(encoded_b));

    let fox = "The quick brown fox jumps over the lazy dog.";
    let encoded_fox = encoding(&fox.as_bytes());
    println!("{:?}", fox.as_bytes());
    println!("{:?}", encoded_fox);
    println!("{:?}", decoding(&encoded_fox));
}