use disco5::computer::*;
use speedy2d::Window;

#[test]
fn test_nes_game() {
    let mut computer: Computer = Default::default();

    computer
        .load_nes_rom(&String::from("sample_programs/Donkey Kong.nes"), 0x8000)
        .unwrap(); // NOTE: verifies that program loaded without errors

    assert_eq!(
        &computer.address_space.ppu.memory[..0x20],
        &[
            0x00, 0x03, 0x07, 0x07, 0x09, 0x09, 0x1c, 0x00, 0x00, 0x03, 0x07, 0x00, 0x06, 0x06,
            0x03, 0x03, 0x0f, 0x0f, 0x0f, 0xff, 0xff, 0xfc, 0x81, 0x01, 0x00, 0x10, 0x3c, 0x3f,
            0x3f, 0x3c, 0x00, 0x00,
        ]
    );

    assert_eq!(
        &computer.address_space.bytes[0xbfe0..=0xbfff],
        &[
            0x56, 0x00, 0x09, 0x07, 0x05, 0x00, 0xca, 0x8a, 0x8a, 0xca, 0xca, 0xce, 0xca, 0xce,
            0xca, 0xce, 0x8e, 0x8e, 0xce, 0xce, 0xd2, 0xce, 0xd2, 0xce, 0x00, 0xff, 0x5f, 0xc8,
            0x9e, 0xc7, 0xf0, 0xff,
        ]
    );

    assert_eq!(
        &computer.address_space.bytes[0xbfe0..=0xbfff],
        &computer.address_space.bytes[0xffe0..=0xffff],
    );

    // let closure = |num: u16| -> bool { false };
    // computer.run_program(false, closure);

    let window = Window::new_centered("Disco5", (256, 240)).unwrap();
    window.run_loop(computer);


    // let closure = |num: u16| -> bool { num == 0x336d };
    // computer.run_program(false, closure);

    // assert_eq!(computer.cpu.pc, 0x336d);
}
