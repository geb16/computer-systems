// src/main.rs
//
// Segmentation (Hands-on Mental Model) — software simulation.
//
// What this simulates (conceptually like an MMU):
// - You have "physical memory" (a byte array).
// - Each process has a Segment Table: entries = {base, limit, permissions}.
// - A logical address is (segment_id, offset).
// - Translation rules:
//     1) offset MUST be < limit      (bounds check => prevents segment overflow)
//     2) permission MUST allow op    (protection check => prevents illegal access)
//     3) physical_address = base + offset
//
// If any rule fails, a real CPU would raise an exception (e.g., segmentation fault).
// Here we return an error with a clear message.
//
// We also print a simple "external fragmentation" view of free holes,
// which is a classic downside of variable-sized segments requiring contiguous placement.

use std::fmt;

#[derive(Clone, Copy, Debug)]
struct SegPerm {
    read: bool,
    write: bool,
    exec: bool,
}
// NOTE: In a real system, permissions would be encoded in bits (e.g., R=4, W=2, X=1) for compactness.

#[derive(Clone, Copy, Debug)] 
// drive means: we want to print these permissions in error messages, so we derive Debug for easy formatting.
struct Segment {
    name: &'static str,
    base: usize,  // physical start address
    limit: usize, // size in bytes
    perm: SegPerm,
}

#[derive(Debug)]
enum SegError {
    InvalidSegment { seg: usize },
    BoundsFault { seg_name: &'static str, offset: usize, limit: usize },
    PermissionFault { seg_name: &'static str, op: &'static str },
    PhysicalOutOfRange { phys: usize, mem_size: usize },
}

impl fmt::Display for SegError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SegError::InvalidSegment { seg } => write!(f, "Invalid segment id: {seg}"),
            SegError::BoundsFault { seg_name, offset, limit } => write!(
                f,
                "Bounds fault: segment '{seg_name}' offset {offset} >= limit {limit}"
            ),
            SegError::PermissionFault { seg_name, op } => {
                write!(f, "Permission fault: '{op}' not allowed on segment '{seg_name}'")
            }
            SegError::PhysicalOutOfRange { phys, mem_size } => write!(
                f,
                "Physical address out of range: {phys} (mem size {mem_size})"
            ),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Read,
    Write,
    Exec,
}

fn check_perm(seg: &Segment, op: Op) -> Result<(), SegError> {
    let allowed = match op {
        Op::Read => seg.perm.read,
        Op::Write => seg.perm.write,
        Op::Exec => seg.perm.exec,
    };
    if allowed {
        Ok(())
    } else {
        let op_str = match op {
            Op::Read => "READ",
            Op::Write => "WRITE",
            Op::Exec => "EXEC",
        };
        Err(SegError::PermissionFault {
            seg_name: seg.name,
            op: op_str,
        })
    }
}

fn translate(
    mem_size: usize,
    segs: &[Segment],
    seg_id: usize,
    offset: usize,
    op: Op,
) -> Result<usize, SegError> {
    let seg = *segs.get(seg_id).ok_or(SegError::InvalidSegment { seg: seg_id })?;

    // 1) Protection check (like hardware permission bits)
    check_perm(&seg, op)?;

    // 2) Bounds check (base/limit like classic segmentation)
    if offset >= seg.limit {
        return Err(SegError::BoundsFault {
            seg_name: seg.name,
            offset,
            limit: seg.limit,
        });
    }

    // 3) Translate to physical address
    let phys = seg.base + offset;

    if phys >= mem_size {
        return Err(SegError::PhysicalOutOfRange { phys, mem_size });
    }
    Ok(phys)
}

fn read_u8(mem: &[u8], segs: &[Segment], seg_id: usize, offset: usize) -> Result<u8, SegError> {
    let phys = translate(mem.len(), segs, seg_id, offset, Op::Read)?;
    Ok(mem[phys])
}

fn write_u8(mem: &mut [u8], segs: &[Segment], seg_id: usize, offset: usize, v: u8) -> Result<(), SegError> {
    let phys = translate(mem.len(), segs, seg_id, offset, Op::Write)?;
    mem[phys] = v;
    Ok(())
}

fn main() {
    // "Physical memory" — 128 bytes for a tiny demo.
    let mut mem = vec![0u8; 128];

    // Segment table like: CODE, DATA, STACK
    //
    // NOTE: Segments require contiguous placement in physical memory (classic segmentation).
    // That’s why external fragmentation can happen over time.
    let segs = [
        Segment {
            name: "CODE",
            base: 0,
            limit: 32, // bytes [0..31]
            perm: SegPerm { read: true, write: false, exec: true },
        },
        Segment {
            name: "DATA",
            base: 40,
            limit: 24, // bytes [40..63]
            perm: SegPerm { read: true, write: true, exec: false },
        },
        Segment {
            name: "STACK",
            base: 80,
            limit: 16, // bytes [80..95]
            perm: SegPerm { read: true, write: true, exec: false },
        },
    ];

    // Seed some bytes in CODE (readable/executable, not writable)
    for i in 0..segs[0].limit {
        mem[segs[0].base + i] = (i as u8).wrapping_add(1);
    }

    println!("=== Segmentation Demo: logical (segment, offset) => physical ===");

    // ✅ Valid read from CODE
    let b = read_u8(&mem, &segs, 0, 10).unwrap();
    println!("READ CODE[10] => {b}  (valid: within limit, READ allowed)");

    // ✅ Valid write to DATA
    write_u8(&mut mem, &segs, 1, 3, 0xAB).unwrap();
    let b2 = read_u8(&mem, &segs, 1, 3).unwrap();
    println!("WRITE+READ DATA[3] => 0x{b2:02X} (valid: WRITE allowed)");

    // ✅ Valid execute from CODE (simulate jumping to an instruction)
    match translate(mem.len(), &segs, 0, 4, Op::Exec) {
        Ok(phys) => println!("EXEC CODE[4] => physical {phys} (valid: EXEC allowed)"),
        Err(e) => println!("Unexpected EXEC fault on CODE: {e}"),
    }

    // ❌ Permission fault: attempt to write CODE
    match write_u8(&mut mem, &segs, 0, 5, 0xFF) {
        Ok(_) => println!("Unexpected: wrote to CODE"),
        Err(e) => println!("EXPECTED FAULT (write CODE): {e}"),
    }

    // ❌ Permission fault: attempt to execute DATA
    match translate(mem.len(), &segs, 1, 2, Op::Exec) {
        Ok(phys) => println!("Unexpected: executed DATA at physical {phys}"),
        Err(e) => println!("EXPECTED FAULT (exec DATA): {e}"),
    }

    // ❌ Bounds fault: offset beyond STACK limit
    match read_u8(&mem, &segs, 2, 99) {
        Ok(v) => println!("Unexpected: read STACK => {v}"),
        Err(e) => println!("EXPECTED FAULT (stack bounds): {e}"),
    }

    // ---- External fragmentation concept (toy view) ----
    //
    // Physical memory map (0..127):
    //   CODE  : [0..31]
    //   HOLE  : [32..39]   (8 bytes free)
    //   DATA  : [40..63]
    //   HOLE  : [64..79]   (16 bytes free)
    //   STACK : [80..95]
    //   HOLE  : [96..127]  (32 bytes free)
    //
    // Imagine a new segment of size 40 bytes arrives.
    // Total free = 8 + 16 + 32 = 56 bytes (enough in total),
    // BUT no single contiguous hole is 40 bytes => cannot place it without compaction.
    println!("\n=== External Fragmentation (Concept) ===");
    println!("Holes: [32..39]=8 bytes, [64..79]=16 bytes, [96..127]=32 bytes");
    println!("Request: NEW_SEG size=40 bytes");
    println!("Result: FAIL in classic segmentation (needs contiguous space); compaction required.\n");

    println!("Done.");
}
