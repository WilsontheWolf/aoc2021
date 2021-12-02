use std::fs;

pub fn get_input() -> String {
    fs::read_to_string("./input2.txt").expect("Something went wrong reading the file")
}

pub fn main() {
    let input = get_input();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut y = 0;
    let mut x = 0;
    for instruction in lines.iter() {
        let split = instruction.split(" ").collect::<Vec<&str>>();
        let direction = split[0];
        let distance = split[1].parse::<i32>().unwrap();
        if direction == "forward" {
            x += distance;
        }
        if direction == "down" {
            y += distance;
        }
        if direction == "up" {
            y -= distance;
        }
    }
    println!("{:?}", y * x);
}
