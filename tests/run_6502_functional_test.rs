use disco5::computer::*;

#[test]
fn test_6502_functional() {
    let mut computer: Computer = Default::default();

    computer
        .load_program_from_hex(
            &String::from("sample_programs/6502_functional_test.bin"),
            0x000a,
            0x400,
        )
        .unwrap(); // NOTE: verifies that program loaded without errors

    assert_eq!(
        &computer.memory[0x400..0x410],
        &[
            0xd8, 0xa2, 0xff, 0x9a, 0xa9, 0x00, 0x8d, 0x00, 0x02, 0xa2, 0x05, 0x4c, 0x33, 0x04,
            0xa0, 0x05
        ]
    );

    let closure = |num: u16| -> bool { num == 0x336d };
    computer.run_program(false, closure);

    assert_eq!(computer.cpu.pc, 0x336d);
}
