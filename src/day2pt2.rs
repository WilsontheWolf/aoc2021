use std::fs;

pub fn get_input() -> String {
    fs::read_to_string("./input2.txt").expect("Something went wrong reading the file")
}

pub fn main() {
    let input = get_input();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut y = 0;
    let mut x = 0;
    let mut aim = 0;
    for instruction in lines.iter() {
        let split = instruction.split(" ").collect::<Vec<&str>>();
        let direction = split[0];
        let distance = split[1].parse::<i32>().unwrap();
        if direction == "forward" {
            x += distance;
            y += distance * aim;
        }
        if direction == "down" {
            aim += distance;
        }
        if direction == "up" {
            aim -= distance;
        }
    }
    println!("{:?}", y * x);
}
