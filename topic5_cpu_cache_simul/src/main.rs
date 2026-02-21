// use std::fmt;

// const RAM_SIZE_BYTES: usize = 64;
// const CACHE_SIZE_BYTES: usize = 16;
// const CACHE_LINE_BYTES: usize = 4;
// const ARRAY_LEN: usize = RAM_SIZE_BYTES / CACHE_LINE_BYTES;
// const CACHE_LINES: usize = CACHE_SIZE_BYTES / CACHE_LINE_BYTES;
// const OFFSET_BITS: u8 = 2;
// const INDEX_BITS: u8 = 2;

// #[derive(Clone, Copy, Debug)]
// struct CacheLine {
//     valid: bool,
//     tag: u8,
//     value: i32,
// }

// impl Default for CacheLine {
//     fn default() -> Self {
//         Self {
//             valid: false,
//             tag: 0,
//             value: 0,
//         }
//     }
// }

// struct DirectMappedCache {
//     lines: [CacheLine; CACHE_LINES],
// }

// impl DirectMappedCache {
//     fn new() -> Self {
//         Self {
//             lines: [CacheLine::default(); CACHE_LINES],
//         }
//     }

//     fn access(&mut self, address: usize, ram: &[i32; ARRAY_LEN]) -> AccessResult {
//         let parts = AddressParts::from(address);
//         let cache_index = parts.index as usize;
//         let block_number = address / CACHE_LINE_BYTES;
//         let value = ram[block_number];
//         let line = &mut self.lines[cache_index];

//         if line.valid && line.tag == parts.tag {
//             return AccessResult {
//                 event: CacheEvent::Hit,
//                 parts,
//                 cache_index,
//                 block_range: block_byte_range(address),
//                 value,
//             };
//         }

//         let miss_kind = if !line.valid {
//             MissKind::Cold
//         } else {
//             MissKind::Conflict
//         };

//         *line = CacheLine {
//             valid: true,
//             tag: parts.tag,
//             value,
//         };

//         AccessResult {
//             event: CacheEvent::Miss(miss_kind),
//             parts,
//             cache_index,
//             block_range: block_byte_range(address),
//             value,
//         }
//     }
// }

// #[derive(Clone, Copy)]
// struct AddressParts {
//     tag: u8,
//     index: u8,
//     offset: u8,
//     binary: [char; 6],
// }

// impl AddressParts {
//     fn from(address: usize) -> Self {
//         let tag = (address >> (OFFSET_BITS + INDEX_BITS)) as u8;
//         let index = ((address >> OFFSET_BITS) & ((1 << INDEX_BITS) - 1)) as u8;
//         let offset = (address & ((1 << OFFSET_BITS) - 1)) as u8;
//         let binary = to_binary_array(address as u8);
//         Self {
//             tag,
//             index,
//             offset,
//             binary,
//         }
//     }
// }

// fn to_binary_array(value: u8) -> [char; 6] {
//     let mut arr = ['0'; 6];
//     for bit in 0..6 {
//         let mask = 1 << (5 - bit);
//         arr[bit] = if value & mask != 0 { '1' } else { '0' };
//     }
//     arr
// }

// fn block_byte_range(address: usize) -> (usize, usize) {
//     let start = address & !(CACHE_LINE_BYTES - 1);
//     let end = start + CACHE_LINE_BYTES - 1;
//     (start, end)
// }

// struct AccessResult {
//     event: CacheEvent,
//     parts: AddressParts,
//     cache_index: usize,
//     block_range: (usize, usize),
//     value: i32,
// }

// enum CacheEvent {
//     Hit,
//     Miss(MissKind),
// }

// impl fmt::Display for CacheEvent {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             CacheEvent::Hit => write!(f, "HIT"),
//             CacheEvent::Miss(kind) => write!(f, "MISS ({kind})"),
//         }
//     }
// }

// #[derive(Clone, Copy)]
// enum MissKind {
//     Cold,
//     Conflict,
// }

// impl fmt::Display for MissKind {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             MissKind::Cold => write!(f, "cold"),
//             MissKind::Conflict => write!(f, "conflict"),
//         }
//     }
// }

// fn main() {
//     println!("Low-Level CPU Cache Simulation\n");
//     print_system_config();

//     let ram = init_ram();
//     print_ram_layout(&ram);

//     let mut cache = DirectMappedCache::new();
//     let mut total_sum = 0;

//     println!("\nStep 4 — First sequential pass (arr[0..4])");
//     total_sum += run_pattern(
//         &mut cache,
//         &ram,
//         &[0, 1, 2, 3],
//         "Cold misses while warming cache",
//     );

//     println!("\nStep 5 — Second sequential pass (arr[0..4])");
//     total_sum += run_pattern(
//         &mut cache,
//         &ram,
//         &[0, 1, 2, 3],
//         "Hits from temporal locality",
//     );

//     println!("\nStep 6 — Stride pattern [0, 4, 8, 12]");
//     total_sum += run_pattern(
//         &mut cache,
//         &ram,
//         &[0, 4, 8, 12],
//         "Conflict misses caused by identical cache indexes",
//     );

//     println!("Final accumulated sum: {}", total_sum);
//     println!("\nMiss taxonomy: cold (first touch), hit (warm), conflict (thrashing the same index). Capacity misses would appear when the working set exceeds {} bytes even with perfect placement.", CACHE_SIZE_BYTES);
// }

// fn run_pattern(
//     cache: &mut DirectMappedCache,
//     ram: &[i32; ARRAY_LEN],
//     indices: &[usize],
//     label: &str,
// ) -> i32 {
//     println!("  {}", label);
//     let mut subtotal = 0;
//     for &idx in indices {
//         let address = idx * CACHE_LINE_BYTES;
//         let access = cache.access(address, ram);
//         subtotal += access.value;
//         report_access(idx, address, &access);
//     }
//     println!("  Pattern subtotal: {}\n", subtotal);
//     subtotal
// }

// fn report_access(arr_index: usize, address: usize, access: &AccessResult) {
//     let bin: String = access.parts.binary.iter().collect();
//     println!(
//         "    arr[{arr_index:2}] addr {address:02} ({bin}) => tag {:02b}, index {:02b}, offset {:02b} -> {}",
//         access.parts.tag, access.parts.index, access.parts.offset, access.event
//     );
//     if let CacheEvent::Miss(kind) = access.event {
//         let (start, end) = access.block_range;
//         println!(
//             "      load bytes {start:02}-{end:02} into cache line {} (tag {:02b}) [{} miss]",
//             access.cache_index, access.parts.tag, kind
//         );
//     }
// }

// fn init_ram() -> [i32; ARRAY_LEN] {
//     let mut ram = [0; ARRAY_LEN];
//     for (i, slot) in ram.iter_mut().enumerate() {
//         *slot = (i as i32) * 3 + 1;
//     }
//     ram
// }

// fn print_system_config() {
//     println!("System configuration:");
//     println!("  RAM size   : {} bytes", RAM_SIZE_BYTES);
//     println!("  Cache size : {} bytes", CACHE_SIZE_BYTES);
//     println!("  Line size  : {} bytes", CACHE_LINE_BYTES);
//     println!("  Lines      : {} (direct mapped)", CACHE_LINES);
//     println!(
//         "  Address    : [TAG|INDEX|OFFSET] = [{}|{}|{}] bits",
//         6 - (INDEX_BITS + OFFSET_BITS), INDEX_BITS, OFFSET_BITS
//     );
// }

// fn print_ram_layout(ram: &[i32; ARRAY_LEN]) {
//     println!("\nRAM layout (int array):");
//     for i in 0..ARRAY_LEN {
//         let start = i * CACHE_LINE_BYTES;
//         let end = start + CACHE_LINE_BYTES - 1;
//         println!("  arr[{i:2}] = {:>3} -> addresses {start:02}-{end:02}", ram[i]);
//     }
// }

fn main() {
    let ptr = 0x12345678 as *const i32; 
    //means: "treat the integer 0x12345678 as a memory address pointing to an i32 value"
    // 0x12345678 is a hexadecimal literal representing a memory address. 
    // The `as *const i32` part tells Rust to interpret this integer as a 
    //pointer to a constant 32-bit integer (i32).
    // ptr is now a raw pointer of type `*const i32` that points to the memory address 0x12345678.
    unsafe {
        println!("{}", *ptr);
    }
}