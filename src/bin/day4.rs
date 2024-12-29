use std::fs;

fn is_valid_password1(password: u32) -> bool {
    let digits = password
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    if !(0..digits.len() - 1).any(|i| digits[i] == digits[i + 1]) {
        return false;
    }
    if (0..digits.len() - 1).any(|i| digits[i] > digits[i + 1]) {
        return false;
    }
    true
}

fn is_valid_password2(password: u32) -> bool {
    let digits = password
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    if !(0..digits.len() - 1).any(|i| {
        digits[i] == digits[i + 1]
            && (i == 0 || digits[i - 1] != digits[i])
            && (i + 1 == digits.len() - 1 || digits[i + 2] != digits[i])
    }) {
        return false;
    }

    if (0..digits.len() - 1).any(|i| digits[i] > digits[i + 1]) {
        return false;
    }
    true
}

fn part1(input: &str) -> usize {
    let mut s = input.split('-');
    let min = s.next().unwrap().parse::<u32>().unwrap();
    let max = s.next().unwrap().parse::<u32>().unwrap();
    (min..=max).filter(|pw| is_valid_password1(*pw)).count()
}

fn part2(input: &str) -> usize {
    let mut s = input.split('-');
    let min = s.next().unwrap().parse::<u32>().unwrap();
    let max = s.next().unwrap().parse::<u32>().unwrap();
    (min..=max).filter(|pw| is_valid_password2(*pw)).count()
}

fn main() {
    let input = fs::read_to_string("input/day4.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
