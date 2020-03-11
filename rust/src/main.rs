#[macro_use]
extern crate contracts;

mod chip_8;
mod design_by_contract;
mod roman_numerals;
mod solitaire;
mod text_wrap;

fn main() {
    use chip_8::Chip8;
    print!("{}[2J", 27 as char);
    let mut chip8 = Chip8::new();
    chip8.load_rom("./src/maze.ch8");
    chip8.run();
}
