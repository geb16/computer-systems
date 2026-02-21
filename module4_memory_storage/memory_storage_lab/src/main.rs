use std::mem;

/// Pretty-print bytes as hex like: "78 56 34 12"
fn hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02X}", b)) // :02x
        .collect::<Vec<_>>()
        .join(" ")
}

/// Build a demo binary record:
/// [magic: u32 LE][temp_tenths_c: i16 LE][voltage: f32 bits LE][name_len: u8][name_bytes UTF-8]
fn build_record(magic: u32, temp_tenths_c: i16, voltage: f32, name: &str) -> Vec<u8> {
    let mut out = Vec::new();

    // Endianness is explicit and portable:
    out.extend_from_slice(&magic.to_le_bytes());

    // Signed integer still becomes bytes; interpretation is done on decode.
    out.extend_from_slice(&temp_tenths_c.to_le_bytes());

    // IEEE 754 float: we serialize its bit pattern (u32) into LE bytes.
    out.extend_from_slice(&voltage.to_bits().to_le_bytes());

    // UTF-8: String is bytes. Length is bytes length, not "characters count".
    let name_bytes = name.as_bytes();
    assert!(name_bytes.len() <= 255);
    out.push(name_bytes.len() as u8);
    out.extend_from_slice(name_bytes);

    out
}

#[derive(Debug)]
#[allow(dead_code)]
struct Record {
    magic: u32,
    temp_tenths_c: i16,
    voltage: f32,
    name: String,
}

/// Parse the record back out of bytes.
fn parse_record(buf: &[u8]) -> Result<Record, String> {
    if buf.len() < 4 + 2 + 4 + 1 {
        return Err("buffer too small".into());
    }

    let magic = u32::from_le_bytes(buf[0..4].try_into().unwrap());
    let temp_tenths_c = i16::from_le_bytes(buf[4..6].try_into().unwrap());

    // voltage is stored as IEEE 754 bits (u32), then reconstructed to f32.
    let voltage_bits = u32::from_le_bytes(buf[6..10].try_into().unwrap());
    let voltage = f32::from_bits(voltage_bits);

    let name_len = buf[10] as usize;
    let start = 11;
    let end = start + name_len;

    if buf.len() < end {
        return Err("buffer too small for name".into());
    }

    // UTF-8 validation happens here.
    let name = std::str::from_utf8(&buf[start..end])
        .map_err(|e| format!("invalid UTF-8: {e}"))?
        .to_string();

    Ok(Record {
        magic,
        temp_tenths_c,
        voltage,
        name,
    })
}

fn main() {
    println!("--- Machine basics ---");
    println!("size_of::<u8>()  = {}", mem::size_of::<u8>());
    println!("size_of::<u32>() = {}", mem::size_of::<u32>());
    println!("size_of::<usize>() (pointer-sized) = {}", mem::size_of::<usize>());
    println!("target_endian = {}", if cfg!(target_endian = "little") { "little" } else { "big" });
    println!();

    println!("--- Endianness demo (u32) ---");
    let x: u32 = 0x12_34_56_78;
    let le = x.to_le_bytes();
    let be = x.to_be_bytes();
    println!("x = 0x12345678");
    println!("LE bytes: {}", hex(&le));
    println!("BE bytes: {}", hex(&be));
    println!();

    println!("--- Two's complement demo (i8) ---");
    let a: i8 = -6;
    // Casting i8 -> u8 preserves the underlying 8-bit pattern.
    let a_bits: u8 = a as u8;
    println!("a = -6 as i8");
    println!("bit pattern (as u8 hex) = 0x{:02X}", a_bits);
    println!("That 0xFA is -6 in two's complement for 8 bits.");
    println!();

    println!("--- Float bits demo (IEEE 754) ---");
    let f: f32 = 3.1415926;
    let bits = f.to_bits();
    println!("f = {f}");
    println!("f bits (u32 hex) = 0x{bits:08X}");
    println!("f bytes (LE) = {}", hex(&bits.to_le_bytes()));
    println!();

    println!("--- UTF-8 string demo ---");
    let s = "DJ ✅"; // note: checkmark is multi-byte in UTF-8
    println!("s = {s}");
    println!("s.len() bytes = {}", s.len());
    println!("s.as_bytes()  = {}", hex(s.as_bytes()));
    println!();

    println!("--- Pointer demo (addresses) ---");
    let value: u32 = 123;
    let r: &u32 = &value;
    let p: *const u32 = r as *const u32; // raw pointer is "just an address"
    println!("&value (safe ref) points to address: {:p}", r);
    println!("p (raw pointer) same address:       {:p}", p);
    // Dereferencing raw pointers requires unsafe:
    unsafe {
        println!("*p (unsafe deref) = {}", *p);
    }
    println!();

    println!("--- Real-world binary record demo ---");
    let record_bytes = build_record(0xA1B2C3D4, -105, 3.98, "sensor-α");
    println!("serialized record bytes:");
    println!("{}", hex(&record_bytes));

    let parsed = parse_record(&record_bytes).expect("parse ok");
    println!("parsed record: {parsed:?}");

    // Convert tenths °C to real °C
    let temp_c = parsed.temp_tenths_c as f32 / 10.0;
    println!("temperature = {temp_c}°C");
}
