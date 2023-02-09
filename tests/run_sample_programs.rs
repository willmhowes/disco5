use disco5::computer::*;

#[test]
fn countdown_program() {
    let mut computer: Computer = Default::default();

    computer
        .load_program(&String::from("sample_programs/countdown.txt"))
        .unwrap(); // NOTE: verifies that program loaded without errors

    println!("BEFORE: 0600: {:?}", &computer.memory[600..616]);
    println!("BEFORE: 0016: {:?}", &computer.memory[16..32]);

    computer.run_program();

    println!("AFTER : 0016: {:?}", &computer.memory[16..32]);

    assert_eq!(
        &computer.memory[16..32],
        &[10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0, 0, 0]
    );
}
