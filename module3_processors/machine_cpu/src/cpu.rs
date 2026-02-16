pub struct CPU {
    pub regs: [u8; 16],
    pub mem: [u8; 256],
    pub pc: u8,
    pub zero: bool,
    pub running: bool,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            regs: [0; 16],
            mem: [0; 256],
            pc: 0,
            zero: false,
            running: true,
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        for (i, byte) in program.iter().enumerate() {
            self.mem[i] = *byte;
        }
    }

    pub fn step(&mut self) {
        let opcode = self.mem[self.pc as usize] >> 4;
        let reg = self.mem[self.pc as usize] & 0x0F;
        let operand = self.mem[(self.pc + 1) as usize];

        self.pc += 2;

        match opcode {
            0x1 => { // LOAD
                self.regs[reg as usize] = self.mem[operand as usize];
            }
            0x2 => { // LOADIMM
                self.regs[reg as usize] = operand;
            }
            0x3 => { // STORE
                self.mem[operand as usize] = self.regs[reg as usize];
            }
            0x4 => { // ADD
                self.regs[reg as usize] =
                    self.regs[reg as usize].wrapping_add(self.regs[operand as usize]);
                self.zero = self.regs[reg as usize] == 0;
            }
            0x5 => { // SUB
                self.regs[reg as usize] =
                    self.regs[reg as usize].wrapping_sub(self.regs[operand as usize]);
                self.zero = self.regs[reg as usize] == 0;
            }
            0x6 => { // JMP
                self.pc = operand;
            }
            0x7 => { // JZ
                if self.zero {
                    self.pc = operand;
                }
            }
            0xF => { // HALT
                self.running = false;
            }
            _ => panic!("Unknown opcode"),
        }
    }

    pub fn run(&mut self) {
        while self.running {
            self.step();
        }
    }
}
