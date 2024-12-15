use std::fs;

struct Input {
    muls: Vec<(i32, i32)>,
    sum: i32,
}

fn read_input() -> Input {
    let mut input = Input {
        muls: Vec::new(),
        sum: 0,
    };
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        for (idx, _pat) in line.match_indices("mul(") {
            // max three digit numbers, so we can optimize by looking
            // for the pattern in a short slice of idx..idx+12
            let max_end = std::cmp::min(idx+12, line.len());
            println!("*** {}", &line[idx..max_end]);
            if let Some(end) = line[idx..max_end].find(")") {
                if line[idx..max_end].contains(",") {
                    println!("**  {}", &line[idx+4..idx+end]);
                    let numbers = &mut line[idx+4..idx+end].split(",");
                    input.muls.push((
                        numbers.next().unwrap().parse::<i32>().unwrap(),
                        numbers.next().unwrap().parse::<i32>().unwrap(),
                    ));    
                } else {
                    println!("Bad numbers");
                }
            } else {
                println!("Bad match");
            }
        }
    }
    input
}

fn main() {
    let mut input = read_input();
    println!("Found {} muls:\n", input.muls.len());
    for (a, b) in input.muls {
        input.sum += a * b;
        println!("{} * {} = {}", a, b, a * b);
    }
    println!();
    println!("Total sum {}", input.sum);
}
