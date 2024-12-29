use advent_of_code_2019::intcode::IntCode;
use std::fs;

fn part1(input: &String) -> i32 {
    let mut code = IntCode::from_str(input);
    code.program[1] = 12;
    code.program[2] = 2;
    code.run().unwrap()
}

fn part2(input: &String) -> i32 {
    let mut program: Vec<i32> = input.split(',').map(|x| x.parse().unwrap()).collect();
    for noun in 0..100 {
        for verb in 0..100 {
            program[1] = noun;
            program[2] = verb;
            let mut code = IntCode::new(program.clone());
            if code.run().unwrap() == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    -1
}

fn main() {
    let input = fs::read_to_string("input/day2.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
