use std::fs;

pub fn get_input() -> String {
    fs::read_to_string("./input1.txt").expect("Something went wrong reading the file")
}

pub fn main() {
    let input = get_input();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut end = 0;
    for (i, line) in lines.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let num = line.parse::<i32>().unwrap();
        let prev = lines[i - 1].parse::<i32>().unwrap();
        if num > prev {
            end+= 1;
        }
    }
    println!("{:?}", end);
}
