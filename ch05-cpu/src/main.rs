const MEMORY_SIZE: usize = 0x1000;

struct CPU {
    registers: [u8; 16],
    program_counter: usize,
    memory: [u8; MEMORY_SIZE],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let pc = self.program_counter;
        let op_byte_1 = self.memory[pc] as u16;
        let op_byte_2 = self.memory[pc + 1] as u16;

        op_byte_1 << 8 | op_byte_2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.program_counter += 2;

            let c = ((opcode & 0xf000) >> 12) as u8;
            let x = ((opcode & 0x0f00) >> 8) as u8;
            let y = ((opcode & 0x00f0) >> 4) as u8;
            let d = ((opcode & 0x000f) >> 0) as u8;

            let nnn = opcode & 0x0fff;
            let kk = opcode & 0x00ff;

            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg_1 = self.registers[x as usize];
        let arg_2 = self.registers[y as usize];

        let (val, overflow) = arg_1.overflowing_add(arg_2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xf] = 1;
        } else {
            self.registers[0xf] = 0;
        }
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; MEMORY_SIZE],
        program_counter: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;
    mem[0] = 0x80;
    mem[1] = 0x14;
    mem[2] = 0x80;
    mem[3] = 0x24;
    mem[4] = 0x80;
    mem[5] = 0x34;

    cpu.run();

    assert_eq!(cpu.registers[0], 35);
    println!("result: {:?}", cpu.registers);
}
