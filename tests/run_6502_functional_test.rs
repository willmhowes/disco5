use disco5::computer::*;

fn byte_dump(memory: &[u8], line_count: u16) {
    let mut i = 0;
    let mut line_count = line_count;
    for byte in memory {
        if i == 0 {
            print!("{line_count:0>7x} :");
        }
        if i < 15 {
            print!(" {byte:0>2x}");
            i += 1;
        } else {
            println!(" {byte:0>2x}");
            i = 0;
            line_count += 16;
        }
    }
}

#[test]
fn test_6502_functional() {
    let mut computer: Computer = Default::default();

    computer
        .load_program_from_hex(&String::from("sample_programs/6502_functional_test.bin"))
        .unwrap(); // NOTE: verifies that program loaded without errors

    byte_dump(&computer.memory[0x400..0x410], 0x400);
    // println!("BEFORE: 0016: {:?}", &computer.memory[16..32]);

    computer.run_program(true);

    // println!("AFTER : 0016: {:?}", &computer.memory[16..32]);

    // assert_eq!(
    //     &computer.memory[16..32],
    //     &[10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0, 0, 0]
    // );
}
