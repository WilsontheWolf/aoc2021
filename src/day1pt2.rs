use std::fs;

pub fn get_input() -> String {
    fs::read_to_string("./input1.txt").expect("Something went wrong reading the file")
}

pub fn main() {
    let input = get_input();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut end = 0;
    for (i, line) in lines.iter().enumerate() {
        if i == 0 || lines.iter().count() < i + 3 {
            continue;
        }
        let cur1 = line.parse::<i32>().unwrap();
        let cur2 = lines[i + 1].parse::<i32>().unwrap();
        let cur3 = lines[i + 2].parse::<i32>().unwrap();
        let prev_num = lines[i - 1].parse::<i32>().unwrap();
        let num = cur1 + cur2 + cur3;
        let prev = prev_num + cur1 + cur2;
        if num > prev {
            end+= 1;
        }
    }
    println!("{:?}", end);
}
