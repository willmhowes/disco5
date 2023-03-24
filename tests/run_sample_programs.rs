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
fn countdown_program() {
    let mut computer: Computer = Default::default();

    computer
        .load_program(&String::from("sample_programs/countdown.txt"))
        .unwrap(); // NOTE: verifies that program loaded without errors

    assert_eq!(
        &computer.memory[600..616],
        &[
            0xa2, 0x10, 0xa0, 0x0a, 0x94, 0x00, 0xe8, 0x88, 0xc0, 0x00, 0xd0, 0xf8, 0x00, 0x00,
            0x00, 0x00
        ]
    );

    let closure = |num: u16| -> bool { num == 0x0264 };
    computer.run_program(false, closure);

    assert_eq!(
        &computer.memory[16..32],
        &[10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0, 0, 0]
    );
}
