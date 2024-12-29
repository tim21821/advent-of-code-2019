use advent_of_code_2019::utils;
use std::collections::{HashMap, HashSet};
use std::ops::Add;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Point {
    fn get_manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

const UP: Point = Point { x: 0, y: -1 };
const DOWN: Point = Point { x: 0, y: 1 };
const LEFT: Point = Point { x: -1, y: 0 };
const RIGHT: Point = Point { x: 1, y: 0 };

fn get_wire(instructions: &String) -> HashSet<Point> {
    let mut wire: HashSet<Point> = HashSet::new();
    let mut current = Point { x: 0, y: 0 };
    for instruction in instructions.split(',') {
        match instruction.chars().nth(0).unwrap() {
            'U' => {
                for _ in 0..(instruction[1..].parse::<i32>().unwrap()) {
                    current = current + UP;
                    wire.insert(current);
                }
            }
            'D' => {
                for _ in 0..(instruction[1..].parse::<i32>().unwrap()) {
                    current = current + DOWN;
                    wire.insert(current);
                }
            }
            'L' => {
                for _ in 0..(instruction[1..].parse::<i32>().unwrap()) {
                    current = current + LEFT;
                    wire.insert(current);
                }
            }
            'R' => {
                for _ in 0..(instruction[1..].parse::<i32>().unwrap()) {
                    current = current + RIGHT;
                    wire.insert(current);
                }
            }
            _ => panic!("Unknown instruction {}", instruction),
        }
    }
    wire
}

fn get_wire_steps(instructions: &String) -> HashMap<Point, i32> {
    let mut wire: HashMap<Point, i32> = HashMap::new();
    let mut current = Point { x: 0, y: 0 };
    let mut steps = 0;
    for instruction in instructions.split(',') {
        match instruction.chars().nth(0).unwrap() {
            'U' => {
                for _ in 0..(instruction[1..].parse::<i32>().unwrap()) {
                    current = current + UP;
                    steps += 1;
                    if !wire.contains_key(&current) {
                        wire.insert(current, steps);
                    }
                }
            }
            'D' => {
                for _ in 0..(instruction[1..].parse::<i32>().unwrap()) {
                    current = current + DOWN;
                    steps += 1;
                    if !wire.contains_key(&current) {
                        wire.insert(current, steps);
                    }
                }
            }
            'L' => {
                for _ in 0..(instruction[1..].parse::<i32>().unwrap()) {
                    current = current + LEFT;
                    steps += 1;
                    if !wire.contains_key(&current) {
                        wire.insert(current, steps);
                    }
                }
            }
            'R' => {
                for _ in 0..(instruction[1..].parse::<i32>().unwrap()) {
                    current = current + RIGHT;
                    steps += 1;
                    if !wire.contains_key(&current) {
                        wire.insert(current, steps);
                    }
                }
            }
            _ => panic!("Unknown instruction {}", instruction),
        }
    }
    wire
}

fn part1(lines: &Vec<String>) -> i32 {
    let wire1 = get_wire(&lines[0]);
    let wire2 = get_wire(&lines[1]);

    let mut min_distance = i32::MAX;
    for intersection in wire1.intersection(&wire2) {
        min_distance = i32::min(
            min_distance,
            Point { x: 0, y: 0 }.get_manhattan_distance(intersection),
        );
    }
    min_distance
}

fn part2(lines: &Vec<String>) -> i32 {
    let wire1 = get_wire_steps(&lines[0]);
    let wire2 = get_wire_steps(&lines[1]);

    let mut min_steps = i32::MAX;
    for point in wire1.keys() {
        if wire2.contains_key(point) {
            min_steps = i32::min(min_steps, wire1[point] + wire2[point]);
        }
    }
    min_steps
}

fn main() {
    let lines = utils::read_lines("input/day3.txt");
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}
