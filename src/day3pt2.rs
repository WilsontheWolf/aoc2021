use std::fs;

pub fn get_input() -> String {
    fs::read_to_string("./input3.txt").expect("Something went wrong reading the file")
}

pub fn main() {
    let input = get_input();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut data: Vec<Vec<&str>> = Vec::new();
    let mut new_data: Vec<Vec<&str>> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let nums = line.split("").collect::<Vec<&str>>();
        new_data.push(Vec::new());
        for (j, num) in nums.iter().enumerate() {
            if num.len() > 0 {
                if i == 0 {
                    data.push(Vec::new());
                }
                data[j - 1].push(num);
                new_data[i].push(num);
            }
        }
    }
    let mut removed: Vec<usize> = Vec::new();
    for (i, row) in data.iter().enumerate() {
        let mut new_data_len = new_data.len();
        for (j, idk) in new_data.iter().enumerate() {
            if removed.contains(&j) {
                new_data_len -= 1;
            }
        }
        if new_data_len <= 1 {
            break;
        }
        let mut ones = 0;
        let mut zeros = 0;
        for (j, num) in row.iter().enumerate() {
            if !removed.contains(&j) {
                if num == &"1" {
                    ones += 1;
                } else {
                    zeros += 1;
                }
            }
        }
        let must_be = if ones >= zeros { "1" } else { "0" };
        for (j, num) in new_data.clone().iter().enumerate() {
            if num[i] != must_be {
                removed.push(j);
            }
        }
    }
    let mut removed_two: Vec<usize> = Vec::new();
    for (i, row) in data.iter().enumerate() {
        let mut new_data_len = new_data.len();
        for (j, idk) in new_data.iter().enumerate() {
            if removed_two.contains(&j) {
                new_data_len -= 1;
            }
        }
        if new_data_len <= 1 {
            break;
        }
        let mut ones = 0;
        let mut zeros = 0;
        for (j, num) in row.iter().enumerate() {
            if !removed_two.contains(&j) {
                if num == &"1" {
                    ones += 1;
                } else {
                    zeros += 1;
                }
            }
        }
        let must_be = if ones < zeros { "1" } else { "0" };
        for (j, num) in new_data.clone().iter().enumerate() {
            if num[i] != must_be {
                removed_two.push(j);
            }
        }
    }
    let mut fin = 1;
    for (j, idk) in new_data.iter().enumerate() {
        if !removed.contains(&j) {
            fin *= isize::from_str_radix(&idk.join(""), 2).unwrap();
        }
    }
    for (j, idk) in new_data.iter().enumerate() {
        if !removed_two.contains(&j) {
            fin *= isize::from_str_radix(&idk.join(""), 2).unwrap();
        }
    }
    println!("{:?}", fin);
}
