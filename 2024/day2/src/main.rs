use std::fs;

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

fn parse_report(report: &Vec<i32>) -> Vec<bool> {
    let mut sign = 0;
    let mut sign_ok = true;
    let mut safety = Vec::new();
    let mut iter = report.iter();
    let mut prev = iter.next().unwrap();
    // Need to push one for the first value so indexing works later
    safety.push(true);
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
        safety.push(sign_ok && diff.abs() > 0 && diff.abs() < 4);
        prev = value;
    }
    //println!();
    safety
}

fn print_report(report: &Vec<i32>) -> () {
    for r in report {
        print!("{} ", r)
    }
    println!();
}

/* Note: the diff vec will have zero in first place always */
fn calc_differences(report: &Vec<i32>) -> Vec<i32> {
    let mut diffs = Vec::new();
    let mut iter = report.iter();
    let mut prev = iter.next().unwrap();
    diffs.push(0);
    while let Some(value) = iter.next() {
        diffs.push(prev-value);
        prev = value;
    }
    diffs
}

fn try_dampen(report: &Vec<i32>) -> Vec<i32> {
    let mut dampened = report.clone();
    /* remove duplicates */
    dampened.dedup();
    if dampened.len() < report.len() {
        return dampened;
    }
    /* If the first element is the problem, we can remove it */
    let first_chunk = report.first_chunk::<2>().unwrap();
    if (first_chunk[0] - first_chunk[1]).abs() > 3 {
        dampened.remove(0);
        return dampened;
    }
    /* Same for the last element */
    let last_chunk = report.last_chunk::<2>().unwrap();
    if (last_chunk[0] - last_chunk[1]).abs() > 3 {
        dampened.remove(dampened.len()-1);
        return dampened;
    }

    let mut index = 0;
    while index < report.len() - 1 {
        let diff1 = report[index] - report[index+1];
        let diff2 = report[index+1] - report[index+2];
        if (diff1 < 0 && diff2 > 0) || (diff1 > 0 && diff2 < 0) {
            dampened.remove(index+2);
            break;
        }
        if diff1.abs() > 3 || diff2.abs() > 3 {
            if diff1.abs() > diff2.abs() {
                dampened.remove(index);
                break;
            } else {
                dampened.remove(index+1);
                break;
            }
        }
            index += 1;
    }
    dampened
}

fn main() {
    let mut input = Input {
        reports: Vec::new(),
        safe: Vec::new(),
    };
    parse_input(&mut input);

    let mut n_safe = 0;
    let mut n_unsafe = 0;
    let mut safe_dampened = 0;
    let mut unsafe_dampened = 0;
    for report in input.reports {
        let mut safety = parse_report(&report);
        let mut safe = safety.iter().filter(|r| !**r).count() == 0;
        let can_dampen = safety.iter().filter(|r| !**r).count() == 1;
        if !safe && can_dampen {
            n_unsafe += 1;
            let dampened = try_dampen(&report);
            safety = parse_report(&dampened);
            safe = safety.iter().filter(|r| !**r).count() == 0;
            if safe {
                safe_dampened += 1;
            } else {
                unsafe_dampened += 1;
                println!("Still unsafe :( ");
                print_report(&report);
                print_report(&dampened);
            }
        } else if safe {
            n_safe += 1;
        } else {
            n_unsafe += 1;
        }
        input.safe.push(safe);
    }

    println!("\n\nSUMMARY:\n");
    println!(" Total\t: {}\n Unsafe\t: {}\n Safe\t: {}",
        input.safe.len(),
        input.safe.iter().filter(|r| !**r).count(),
        input.safe.iter().filter(|r| **r).count());

    println!("\n Naturally safe \t: {}\n Naturally unsafe\t: {}",
        n_safe, n_unsafe);
    println!("\n Dampened safe  \t: {}\n Dampened unsafe\t: {}",
        safe_dampened, unsafe_dampened);
}
