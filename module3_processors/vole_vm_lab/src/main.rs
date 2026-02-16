use std::fmt; // fmt means "formatting" used for implementing Debug trait for Instr struct

/// This is a very simple VM for teaching purposes, not meant to be efficient or feature-rich. 
/// It demonstrates the fetch-decode-execute cycle clearly.
/// A tiny VM with:
/// - 16 registers (u16) means we have R0 to R15, each can hold a 16-bit unsigned integer 
/// - 16-bit instructions with fields: opcode (4 bits), r (4 bits), imm (8 bits)
/// - PC (program counter) is an index into the instruction vector, not a byte address
/// - e.g. of 16-bit instruction is 0x1234 where 0x1 is opcode, 0x2 is r, and 0x34 is imm
/// Instruction set:
/// - LDI r, imm: load immediate value into register r
/// - ADDI r, imm: add immediate to register r
/// - SUBI r, imm: subtract immediate from register r
/// - JZ r, imm: if register r is zero, jump to instruction index imm
/// - JMP imm: jump to instruction index imm
/// - HALT: stop execution
/// - program memory: Vec<u16> instructions
/// - PC: program counter as instruction index
/// * usize vs u16: usize is used for indexing into the instruction vector, 
///    which can be larger than 65535 instructions, while u16 would limit us to 65536 instructions. 
///  * Since this is a teaching VM, we can assume programs are small, 
/// but using usize is more flexible and idiomatic for indexing.
/// ISA (Instruction Set Architecture) is the set of instructions that the VM can execute, 
/// and how they are encoded in the instruction word.

#[derive(Default)] // # for default initialization of VM
struct VM { // VM struct holds the state of our virtual machine
    regs: [u16; 16],
    pc: usize,
    halted: bool,
    steps: u64,
}

/// Decoded instruction fields (from 16-bit word).
#[derive(Clone, Copy)]
struct Instr { // what is a struct? it's a custom data type that can hold multiple related values together.
    opcode: u8,
    r: u8,
    imm: u8,
    raw: u16,
}

impl fmt::Debug for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( // the ! is a macro invocation, it allows us to write formatted strings to the formatter.
            f,
            "Instr {{ raw=0x{:04X}, opcode=0x{:X}, r=R{}, imm={} }}",
            self.raw, self.opcode, self.r, self.imm
        )
    }
}

/// Fetch: read instruction at PC, increment PC.
fn fetch(vm: &mut VM, prog: &[u16]) -> Option<u16> {
    if vm.pc >= prog.len() {
        return None;
    }
    let word = prog[vm.pc];
    vm.pc += 1;
    Some(word) // Some means: we successfully fetched an instruction, and we return it wrapped in Some.
}

/// Decode: split into opcode/reg/imm.
/// opcode means the operation code that tells us what instruction to execute, 
/// r is the register index, and 
/// imm is the immediate value.
fn decode(word: u16) -> Instr { // Instr is the struct we defined above to hold the decoded instruction fields.
    let opcode = ((word >> 12) & 0xF) as u8; 
    // >> 12 means we shift the bits to the right by 12 positions, which leaves us with the top 4 bits of the instruction word.
    // & 0xF means we mask out all but the lowest 4 bits, which gives us the opcode value.
    let r = ((word >> 8) & 0xF) as u8;
    let imm = (word & 0xFF) as u8;
    Instr { opcode, r, imm, raw: word }
}

/// Execute: apply the instruction to VM state.
fn execute(vm: &mut VM, ins: Instr) {
    let r = ins.r as usize;
    match ins.opcode {
        0x1 => { // LDI
            vm.regs[r] = ins.imm as u16;
        }
        0x2 => { // ADDI
            vm.regs[r] = vm.regs[r].wrapping_add(ins.imm as u16); // wrapping_add means it wraps on overflow instead of panicking
        }
        0x3 => { // SUBI
            vm.regs[r] = vm.regs[r].wrapping_sub(ins.imm as u16);
        }
        0x4 => { // JZ r, imm (imm is target instruction index)
            if vm.regs[r] == 0 {
                vm.pc = ins.imm as usize;
            }
        }
        0x5 => { // JMP imm
            vm.pc = ins.imm as usize;
        }
        0xF => { // HALT
            vm.halted = true;
        }
        _ => {
            // Unknown opcode -> treat as halt for safety in this teaching VM
            vm.halted = true;
        }
    }
}

/// Helper to assemble instruction words.
fn asm(opcode: u8, r: u8, imm: u8) -> u16 {
    ((opcode as u16) << 12) | ((r as u16) << 8) | (imm as u16)
}

fn main() {
    // Program: count down R1 from 5 to 0 using a loop, then HALT.
    // This demonstrates a simple loop and conditional jump.
    // 0: LDI  R1, 5
    // 1: SUBI R1, 1
    // 2: JZ   R1, 4   ; if R1 == 0 jump to HALT
    // 3: JMP  1       ; loop
    // 4: HALT
    let prog: Vec<u16> = vec![ // vec! is a macro that creates a vector, which is a growable array type in Rust.
        asm(0x1, 1, 5), // 0x1, 1, 5 means opcode=0x1 (LDI), r=1 (R1), imm=5, so this instruction will load the value 5 into register R1.
        asm(0x3, 1, 1), // 0x3, 1, 1 means opcode=0x3 (SUBI), r=1 (R1), imm=1, so this instruction will subtract 1 from the value in register R1.
        asm(0x4, 1, 4), // 0x4, 1, 4 means opcode=0x4 (JZ), r=1 (R1), imm=4, so this instruction will check if the value in register R1 is zero, and if it is, it will jump to instruction index 4 (which is the HALT instruction).
        asm(0x5, 0, 1), // 0x5, 0, 1 means opcode=0x5 (JMP), r=0 (ignored), imm=1, so this instruction will unconditionally jump to instruction index 1 (the SUBI instruction), creating a loop.
        asm(0xF, 0, 0), // 0xF, 0, 0 means opcode=0xF (HALT), r=0 (ignored), imm=0 (ignored), so this instruction will set the halted flag to true, stopping the VM.
    ];

    let mut vm = VM::default();

    println!("=== VOLE-style VM: Fetch–Decode–Execute Trace ===");
    println!("Initial: PC={} R1={}", vm.pc, vm.regs[1]);
    println!("Program length: {} instructions\n", prog.len());

    while !vm.halted {
        let pc_before_fetch = vm.pc;

        // FETCH
        let word = match fetch(&mut vm, &prog) {
            Some(w) => w,
            None => {
                println!("FETCH: PC out of bounds -> HALT");
                break;
            }
        };

        // DECODE
        let ins = decode(word);

        // EXECUTE (trace before/after key regs)
        let r_before = vm.regs[ins.r as usize];
        execute(&mut vm, ins);
        let r_after = vm.regs[ins.r as usize];

        vm.steps += 1;

        println!(
            "step {:>2} | PC(fetch)={} | {:?} | R{}: {} -> {} | PC(after)={}",
            vm.steps,
            pc_before_fetch,
            ins,
            ins.r,
            r_before,
            r_after,
            vm.pc
        );

        if vm.steps > 1000 {
            println!("Safety stop: too many steps");
            break;
        }
    }

    println!("\nHALTED: steps={} final PC={} final R1={}", vm.steps, vm.pc, vm.regs[1]);
}
