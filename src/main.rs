use crate::cpu::CPU;

mod cpu;

fn main() {
    // let input = include_str!("../data/example1.txt");
    let input = include_str!("../data/input.txt");
    let mut input = input
        .split(',')
        .map(|v| v.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    input[1] = 12;
    input[2] = 2;

    let mut cpu = CPU::new(input);
    cpu.run();

    println!("First value after run: {}", cpu.get_first_value());
}
