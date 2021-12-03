use std::fs;

pub fn get_input() -> String {
    fs::read_to_string("./input3.txt").expect("Something went wrong reading the file")
}

pub fn main() {
    let input = get_input();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut data: Vec<Vec<&str>> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let nums = line.split("").collect::<Vec<&str>>();
        for (j, num) in nums.iter().enumerate() {
            if num.len() > 0 {
                if i == 0 {
                    data.push(Vec::new());
                }
                data[j - 1].push(num);
            }
        }
    }
    let mut gamma: Vec<&str> = Vec::new();
    let mut epsilon: Vec<&str> = Vec::new();
    for row in data {
        let mut ones = 0;
        let len = row.len();
        for num in row {
            if num == "1" {
                ones += 1;
            }
        }
        if ones > len / 2 {
            gamma.push("1");
            epsilon.push("0");
        } else {
            gamma.push("0");
            epsilon.push("1");
        }
    }
    let gamma_dec = isize::from_str_radix(&gamma.join(""), 2).unwrap();
    let epsilon_dec = isize::from_str_radix(&epsilon.join(""), 2).unwrap();
        println!("{:?}", gamma_dec * epsilon_dec);
}
