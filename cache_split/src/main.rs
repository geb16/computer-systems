fn main() {
    // let say we have a 32-bit address and a cache with 64B blocks and 128 sets (4-way associative).
    let address: u32 = 0xDEADBEEF;

    // Cache configuration
    let block_offset_bits: u32 = 6; // 64B block
    let set_index_bits: u32 = 7;    // 128 sets

    // Masksing will be easier if we compute them once
    let block_mask = (1 << block_offset_bits) - 1;
    let set_mask = (1 << set_index_bits) - 1;

    // Extracting the tag, set index, and block offset from the address
    let block_offset = address & block_mask;
    let set_index = (address >> block_offset_bits) & set_mask;
    let tag = address >> (block_offset_bits + set_index_bits);

    println!("Address: 0x{:X}", address);
    println!("Tag: 0x{:X}", tag);
    println!("Set Index: {}", set_index);
    println!("Block Offset: {}", block_offset);
}