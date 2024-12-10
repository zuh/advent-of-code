use std::{cmp, fs};

struct Input {
    reports: Vec<Vec<i32>>,
    safe: Vec<bool>,
}

fn parse_input(input: &mut Input) -> () {
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let iter = line.split_ascii_whitespace();
        input.reports.push(
            Vec::from(
                iter.map(|r| r.parse::<i32>().unwrap()).collect::<Vec<i32>>()
            ));
    }
}

fn main() {
    let mut input = Input {
        reports: Vec::new(),
        safe: Vec::new(),
    };
    parse_input(&mut input);

    for report in input.reports {
        let mut sign = 0;
        let mut sign_ok = true;
        let mut max_diff = 0;
        let mut min_diff = 0;
        let mut iter = report.iter();
        let mut prev = iter.next().unwrap();
        while let Some(value) = iter.next() {
            let diff = prev - value;
            let dir = match diff {
                ..0 => -1,
                1.. => 1,
                0 => {
                    sign_ok = false;
                    0
                }
            };
            if sign == 0 {
                sign = dir;
            } else if sign != dir {
                sign_ok = false;
            }
            max_diff = cmp::max(diff.abs(), max_diff);
            match min_diff {
                0 => min_diff = diff.abs(),
                _ => min_diff = cmp::min(diff.abs(), min_diff),
            }
            print!("{} ", diff);
            prev = value;
        }
        if sign_ok && min_diff > 0 && max_diff < 4 {
            print!(" => {}..{} {} => SAFE\n", min_diff, max_diff, sign_ok);
            input.safe.push(true)
        } else {
            print!(" => {}..{} {} => UNSAFE\n", min_diff, max_diff, sign_ok);
            input.safe.push(false)
        }
    }

    println!("\n\nSUMMARY:\n");
    println!(" Total\t: {}\n Unsafe\t: {}\n Safe\t: {}\n",
    input.safe.len(),
    input.safe.iter().filter(|r| !**r).count(),
    input.safe.iter().filter(|r| **r).count());
}
