use disco5::nes::*;
use speedy2d::Window;

fn main() {
    let mut nes: NES = Default::default();

    nes.load_nrom_128(&String::from("sample_programs/Donkey Kong.nes"), 0x8000)
        .unwrap();

    let window = Window::new_centered("Disco5", (1024, 960)).unwrap();
    window.run_loop(nes);
}
