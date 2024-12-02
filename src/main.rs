use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn sort_list(list: &mut Vec<i32>) {
    list.sort();
}

fn subs_2num(x: i32, y: i32) -> i32 {
    let mut out = x - y;
    if out < 0 {
        out = y - x;
        return out;
    } else {
        return out;
    }
}

fn read_file(path: &str) -> Vec<String> {
    let path = path;
    let mut lines: Vec<String> = Vec::new();

    if let Ok(file) = File::open(path) {
        for line in io::BufReader::new(file).lines() {
            if let Ok(content) = line {
                lines.push(content);
            }
        }
    }
    lines
}

fn split_to_lists(lines: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    for line in lines {
        let split: Vec<&str> = line.split(" ").filter(|s| !s.is_empty()).collect();
        list1.push(split[0].to_string().parse::<i32>().unwrap());
        list2.push(split[1].to_string().trim().parse::<i32>().unwrap());
    }
    (list1, list2)
}

fn main() {
    let lines = read_file("input.txt");
    let (mut list1, mut list2) = split_to_lists(lines);
    println!("{}", list1.len());
    println!("{}", list2.len());
    sort_list(&mut list1);
    sort_list(&mut list2);
    let mut output = 0;
    let zipped = list1.iter().zip(list2.iter());
    for (a, b) in zipped {
        output += subs_2num(*a, *b);
    }
    println!("{}", output);
}
