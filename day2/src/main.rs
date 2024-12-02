mod file_read {
    use std::fs::File;
    use std::io::{self, BufRead};

    pub fn read_file(path: &str) -> Vec<Vec<i32>> {
        let mut reports: Vec<Vec<i32>> = Vec::new();
        let text = read_file_text(path);
        println!("{}", text.len());
        for t in &text {
            let result = transform_to_i32(t.split(" ").collect());
            reports.push(result);
        }
        reports
    }

    fn transform_to_i32(levels: Vec<&str>) -> Vec<i32> {
        let mut out: Vec<i32> = Vec::new();
        for level in levels {
            out.push(level.parse::<i32>().unwrap());
        }
        out
    }

    fn read_file_text(path: &str) -> Vec<String> {
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
}

mod logic_checker {
    pub fn check_if_list_increasing(input_list: &Vec<i32>) -> bool {
        for i in 0..input_list.len() - 1 {
            if input_list[i] > input_list[i + 1] {
                return false;
            }
        }
        return true;
    }

    pub fn check_if_list_decreasing(input_list: &Vec<i32>) -> bool {
        for i in 0..input_list.len() - 1 {
            if input_list[i] < input_list[i + 1] {
                return false;
            }
        }
        return true;
    }

    pub fn test_list(x: &Vec<i32>) -> bool {
        for i in 0..x.len() - 1 {
            let mut out = x[i] - x[i + 1]; // Make `out` mutable so you can modify it
                                           // If two consecutive numbers are equal, return false
            if x[i] == x[i + 1] {
                return false;
            }
            // If the difference is negative, make it positive
            if out < 0 {
                out = -out;
            }
            // If the difference is not 1, 2, or 3, return false
            if out != 1 && out != 2 && out != 3 {
                return false;
            }
        }
        // If the loop finishes without returning, return true
        true
    }

    // def test_list(x: list) -> bool:
    //     x_len = len(x)
    //     for i in range(x_len - 1):
    //         if x[i] == x[i + 1]:
    //             return False
    //         out = x[i] - x[i + 1]
    //         if out < 0:
    //             out = -out
    //         if out not in [1, 2, 3]:
    //             return False
    //     return True
}

// fn process_text(text: Vec<String>) -> Vec<Vec<i32>> {
//     for line in text.split(" ") {}
// }
//

fn main() {
    use file_read::read_file;
    use logic_checker::check_if_list_decreasing;
    use logic_checker::check_if_list_increasing;
    use logic_checker::test_list;
    let mut result = 0;
    let datas = read_file("src/input.txt");
    for data in datas {
        if check_if_list_increasing(&data) || check_if_list_decreasing(&data) {
            if test_list(&data) {
                result += 1
            }
        }
    }
    println!("{}", result);
}
