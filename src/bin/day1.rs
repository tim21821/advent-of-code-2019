use advent_of_code_2019::utils;

fn get_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut fuel_sum = 0;
    for line in lines {
        let mass = line.parse::<i32>().unwrap();
        fuel_sum += get_fuel(mass);
    }

    fuel_sum
}

fn part2(lines: &Vec<String>) -> i32 {
    let mut fuel_sum = 0;
    for line in lines {
        let mass = line.parse::<i32>().unwrap();
        let mut fuel_needed = get_fuel(mass);
        while fuel_needed > 0 {
            fuel_sum += fuel_needed;
            fuel_needed = get_fuel(fuel_needed);
        }
    }

    fuel_sum
}

fn main() {
    let lines = utils::read_lines("input/day1.txt");
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}
