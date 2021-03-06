fn abs(num: i8) -> u8 {
    if num < 0 {
        -num as u8
    } else {
        num as u8
    }
}

pub struct Chip8 {
    memory: Vec<u8>,
    registers: Vec<u8>,
    i_register: u16,
    stack: Vec<u8>,
    delay_timer: u16,
    sound_timer: u16,
    input: u8,
    opcodes: Vec<u16>,
    pc: u16,
    vram: Vec<Vec<u8>>,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: vec![0; 0x1000],
            registers: vec![0; 0x10],
            i_register: 0,
            stack: vec![0; 0x30],
            delay_timer: 0,
            sound_timer: 0,
            input: 0,
            opcodes: Vec::new(),
            pc: 0,
            vram: vec![vec![0; 64]; 32],
        }
    }

    pub fn load_rom(&mut self, path: &str) {
        use std::fs;

        let bytes = fs::read(path).unwrap();
        for i in 0..bytes.len() {
            self.memory[i + 512] = bytes[i];
        }
        let mut opcodes: Vec<u16> = Vec::new();
        for chunk in bytes.chunks(2) {
            let opcode = ((chunk[0] as u16) << 8) | chunk[1] as u16;
            opcodes.push(opcode);
        }
    }

    pub fn run(&mut self) {
        self.pc = 512;
        // println!("{}", self.memory[self.pc as usize + 2]);
        while self.pc < self.memory.len() as u16 {
            let high = self.memory[self.pc as usize];
            let low = self.memory[self.pc as usize + 1];
            let opcode = ((high as u16) << 8) | low as u16;
            // println!("running opcode: {:X}", opcode);
            self.run_opcode(opcode);
            self.pc += 2;
            std::thread::sleep_ms(35);
        }
    }

    fn draw(&mut self, x: u16, y: u16, n: u8) {
        let x = self.registers[x as usize];
        let y = self.registers[y as usize];

        for i in 0..n {
            let byte = self.memory[(self.i_register + i as u16) as usize];
            let bit_coded = format!("{:0>8}", format!("{:b}", byte));
            let bit_coded = bit_coded
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>();

            for j in 0..bit_coded.len() {
                match bit_coded[j] {
                    0 => (),
                    1 => {
                        self.vram[(y + i) as usize][(x + j as u8) as usize] =
                            abs(self.vram[(y + i) as usize][(x + j as u8) as usize] as i8 - 1);
                        self.registers[0xF] = 1;
                    }
                    _ => panic!("invalid bit!"),
                }
            }
        }

        // self.print();
    }

    fn print(&self) {
        print!("{}[2J", 27 as char);
        for row in &self.vram {
            for pixel in row {
                match pixel {
                    0 => print!(" "),
                    1 => print!("\x1b[41;1m \x1b[0m"),
                    _ => panic!("invalid bit!"),
                }
            }

            print!("\n");
        }
    }

    fn subroutine(&mut self, address: u16) {
        let high = self.memory[address as usize];
        let low = self.memory[(address + 1) as usize];
        let opcode = ((high as u16) << 8) | low as u16;
        self.run_opcode(opcode);
    }

    fn run_opcode(&mut self, opcode: u16) {
        let nnn = opcode & 0x0FFF;
        let nn = opcode as u8;
        let opcode = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8,
        );
        let x = opcode.1;
        let y = opcode.2;
        let n = opcode.3;

        match opcode {
            (0x1, _, _, _) => self.pc = nnn,
            (0x2, _, _, _) => self.subroutine(nnn),
            (0x3, _, _, _) => {
                if self.registers[x as usize] == nn {
                    self.pc += 2;
                }
            }
            (0x4, _, _, _) => {
                if self.registers[x as usize] != nn {
                    self.pc += 2;
                }
            }
            (0x5, _, _, 0x0) => {
                if self.registers[x as usize] == self.registers[y as usize] {
                    self.pc += 2;
                }
            }
            (0x7, _, _, _) => {
                let vx = self.registers[x as usize] as u16;
                let val = nn as u16;
                let result = vx + val;
                self.registers[x as usize] = result as u8;
            }
            (0x8, _, _, 0) => self.registers[x as usize] = self.registers[y as usize],
            (0x8, _, _, 1) => {
                self.registers[x as usize] = self.registers[x as usize] | self.registers[y as usize]
            }
            (0x8, _, _, 2) => {
                self.registers[x as usize] = self.registers[x as usize] & self.registers[y as usize]
            }
            (0x8, _, _, 3) => {
                self.registers[x as usize] = self.registers[x as usize] ^ self.registers[y as usize]
            }
            (0x8, _, _, 4) => {
                let sum = self.registers[x as usize] as u16 + self.registers[y as usize] as u16;
                if sum > 255 {
                    self.registers[0xF] = 1;
                }
                self.registers[x as usize] = sum as u8;
            }
            (0x8, _, _, 5) => {
                if self.registers[x as usize] < self.registers[y as usize] {
                    self.registers[0xF] = 0;
                } else {
                    self.registers[0xF] = 1;
                }
                self.registers[x as usize] =
                    (self.registers[x as usize] as i16 - self.registers[y as usize] as i16) as u8;
            }
            (0x8, _, _, 6) => {
                let lsb = self.registers[x as usize] & 1;
                self.registers[0xF] = lsb;
                let padding: u16 = 0;
                let double = ((self.registers[x as usize] as u16) << 8) | padding;
                self.registers[x as usize] = (((double >> 1) & 0xF) >> 8) as u8;
            }
            (0x8, _, _, 0xE) => {
                let msb = (self.registers[x as usize] & 0x80) >> 7;
                self.registers[0xF] = msb;
                println!("{:b}", self.registers[x as usize]);
                self.registers[x as usize] = ((self.registers[x as usize] as u16) << 1) as u8;
                println!("{:b}", self.registers[x as usize]);
            }
            (0xF, _, 0x1, 0x5) => self.delay_timer = self.registers[x as usize] as u16,
            (0xF, _, 0x5, 0x5) => {
                let mut i = self.i_register as usize;
                for reg in 0..x as usize {
                    self.memory[i] = self.registers[reg];
                    i += 1;
                }
            }
            (0xF, _, 0x3, 0x3) => {
                let exponents = format!("{:0>3}", self.registers[x as usize]);
                let exponents = exponents
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>();
                let mut i = self.i_register as usize;
                for digit in exponents {
                    self.memory[i] = digit;
                }
            }
            (0xF, _, 0x6, 0x5) => {
                let mut i = self.i_register as usize;
                for reg in 0..x as usize {
                    self.registers[reg] = self.memory[i];
                    i += 1;
                }
            }
            (0xA, _, _, _) => self.i_register = nnn,
            (0xD, _, _, _) => self.draw(x, y, n),
            (0x6, _, _, _) => self.registers[x as usize] = nn,
            (0x9, _, _, 0x0) => {
                if self.registers[x as usize] != self.registers[y as usize] {
                    self.pc += 2;
                }
            }
            (0xC, _, _, _) => {
                extern crate rand;
                use rand::Rng;
                let mut rand = rand::thread_rng();
                let num: u8 = rand.gen_range(0, 255);
                println!("{}", num);
                self.registers[x as usize] = num & nn;
            }
            _ => panic!(
                "Unknown opcode: {:X}{:X}{:X}{:X}",
                opcode.0, opcode.1, opcode.2, opcode.3
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_rom() {
    //     let mut chip8 = Chip8::new();
    //     chip8.load_rom("./src/test_opcode.ch8");
    //     chip8.run();
    //     assert!(false);
    // }

    #[test]
    fn maze_rom() {
        print!("{}[2J", 27 as char);
        let mut chip8 = Chip8::new();
        chip8.load_rom("./src/maze.ch8");
        chip8.run();
        assert!(false);
    }
}
