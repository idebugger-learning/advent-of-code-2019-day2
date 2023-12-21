use crate::cpu::CPU;

mod cpu;

fn main() {
    // let input = include_str!("../data/example1.txt");
    let input = include_str!("../data/input.txt");
    let input = input
        .split(',')
        .map(|v| v.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let expected_result = 19690720;
    for noun in 0..=99 {
        for verb in 0..=99 {
            print!("Trying noun {} and verb {}: ", noun, verb);

            let mut cpu = CPU::new(input.clone());
            let result = cpu.run(noun, verb);

            print!("{}. ", result);
            if result == expected_result {
                println!("✅ Match!");
                print_result(noun, verb);
                return;
            } else {
                println!("Nah.")
            }
        }
    }
}

fn print_result(noun: isize, verb: isize) {
    println!("The answer is {}", 100 * noun + verb)
}
