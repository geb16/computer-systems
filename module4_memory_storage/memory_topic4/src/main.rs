use std::mem; // mem is a module for inspecting types and sizes at compile time, among other things.

fn hex(bytes: &[u8]) -> String {
    bytes
    .iter()
    .map(|b| format!("{:02X}", b)) // |b| means "for each byte b, format it as two-digit uppercase hex". {:02X} means "pad with zeros to 2 digits, and use uppercase letters for A-F".
    .collect::<Vec<_>>()
    .join(" ")
}

// Record format (ALL bytes in this exact order):
// [0..4)  magic u32 (LE) : magic means some fixed value to identify the record type, and can be used for sanity checks.
// [4..6)  temp i16 (LE)  -> tenths of °C
// [6..10) voltage f32 bits (LE) -> IEEE 754
// [10]    name_len u8 [10] means "the byte at index 10", which is a single byte that gives the length of the name string in bytes.
// [11..]  name bytes (UTF-8)
fn build_record(magic: u32, temp_tenths_c: i16, voltage: f32, name: &str) -> Vec<u8> {
    let mut out = Vec::new();

    // u32 -> 4 bytes in little-endian order
    out.extend_from_slice(&magic.to_le_bytes()); //extend_from_slice takes a slice of bytes and appends it to the vector. to_le_bytes() converts the u32 into an array of 4 bytes in little-endian order.

    // i16 -> 2 bytes in little-endian order (two's complement bit pattern)
    out.extend_from_slice(&temp_tenths_c.to_le_bytes());

    // f32 stored as raw IEEE-754 bits (u32), then serialized as LE bytes
    out.extend_from_slice(&voltage.to_bits().to_le_bytes());

    // UTF-8 string bytes + 1-byte length prefix
    let name_bytes = name.as_bytes();
    assert!(name_bytes.len() <= 255, "name must fit in u8 length");
    out.push(name_bytes.len() as u8);
    out.extend_from_slice(name_bytes);

    out
}

#[derive(Debug)]
#[allow(dead_code)] // This attribute tells the Rust compiler to not warn about unused code in this struct, since we only use it for parsing and printing, but not for any logic.
struct Record {
    magic: u32,
    temp_tenths_c: i16,
    voltage: f32,
    name: String,
}

fn parse_record(buf: &[u8]) -> Result<Record, String> {
    // Minimum length: 4 + 2 + 4 + 1 = 11 bytes
    if buf.len() < 11 {
        return Err("buffer too small".into());
    }

    // 1) magic (u32 LE)
    let magic = u32::from_le_bytes(buf[0..4].try_into().unwrap());

    // 2) temperature (i16 LE)
    let temp_tenths_c = i16::from_le_bytes(buf[4..6].try_into().unwrap());

    // 3) voltage (f32 bits LE)
    let voltage_bits = u32::from_le_bytes(buf[6..10].try_into().unwrap());
    let voltage = f32::from_bits(voltage_bits);

    // 4) name length + bytes
    let name_len = buf[10] as usize;
    let start = 11;
    let end = start + name_len;
    if buf.len() < end {
        return Err("buffer too small for name".into());
    }

    // Validate UTF-8 while converting bytes -> String
    let name = std::str::from_utf8(&buf[start..end])
        .map_err(|e| format!("invalid UTF-8: {e}"))?
        .to_string();

    Ok(Record { magic, temp_tenths_c, voltage, name })
}

fn main() {
    // These sizes are facts about your current machine build target
    println!("usize bytes (pointer-sized): {}", mem::size_of::<usize>());
    println!("u32 bytes: {}", mem::size_of::<u32>());
    println!("i16 bytes: {}", mem::size_of::<i16>());
    println!("f32 bytes: {}", mem::size_of::<f32>());
    println!("u8 bytes: {}", mem::size_of::<u8>());
    println!("length of name s.len(): {}", "sensor-α".len()); // .len() gives the byte length of the string, which is 9 bytes for "sensor-α" because 'α' is a 2-byte UTF-8 character.
    println!("endianness: {}", if cfg!(target_endian = "little") { "little" } else { "big" }); 
    // cfg! is a compile-time macro that checks if the target architecture is little-endian or big-endian, and returns a boolean. 
    // We use it to print the endianness of the machine.
    println!();

    // Build bytes (serialization)
    let bytes = build_record(0xA1B2C3D4, -105, 3.98, "sensor-α");
    println!("RAW BYTES (hex):");
    println!("{}", hex(&bytes));
    println!();

    // Parse bytes (deserialization)
    let r = parse_record(&bytes).expect("should parse");
    println!("PARSED STRUCT:");
    println!("{r:?}");

    // Convert tenths °C to °C
    let temp_c = r.temp_tenths_c as f32 / 10.0;
    println!("temperature °C = {}", temp_c);
}
