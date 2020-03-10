fn abs(num: i8) -> u8 {
    if num < 0 {
        -num as u8
    } else {
        num as u8
    }
}

struct Chip8 {
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
    fn new() -> Self {
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

    fn load_rom(&mut self, path: &str) {
        use std::fs;

        let bytes = fs::read(path).unwrap();
        let mut opcodes: Vec<u16> = Vec::new();
        for chunk in bytes.chunks(2) {
            let opcode = ((chunk[0] as u16) << 8) | chunk[1] as u16;
            opcodes.push(opcode);
        }

        self.opcodes = opcodes;
    }

    fn run(&mut self) {
        while self.pc < self.opcodes.len() as u16 {
            println!("RUNNING OPCODE: {:X}", self.opcodes[self.pc as usize]);
            self.run_opcode(self.opcodes[self.pc as usize]);
            self.pc += 1;
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

        self.print();
    }

    fn print(&self) {
        for row in &self.vram {
            for pixel in row {
                print!("{}", pixel);
            }

            print!("\n");
        }
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
            (0x1, _, _, _) => self.pc = nnn - 0x200,
            (0xA, _, _, _) => self.i_register = nnn - 0x200,
            (0xD, _, _, _) => self.draw(x, y, n),
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
    use std::fs;

    #[test]
    fn read_opcode_test() {
        let mut chip8 = Chip8::new();
        chip8.load_rom("./src/test_opcode.ch8");
        chip8.run();
        assert!(false);
    }
}
