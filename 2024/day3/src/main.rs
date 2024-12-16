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
    let mut enabled = true;
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        // First we scan for do'sa and don'ts and collect the indices
        // where the state toggles.
        let dos: Vec<usize> = line.match_indices("do()").map(|(idx, _pat)| idx).collect();
        let donts: Vec<usize> = line.match_indices("don't()").map(|(idx, _pat)| idx).collect();
        let mut active_iter = dos.iter();
        let mut inactive_iter = donts.iter();
        let mut active_idx = *active_iter.next().unwrap();
        let mut inactive_idx = *inactive_iter.next().unwrap();
        for (idx, _pat) in line.match_indices("mul(") {
            if active_idx < idx {
                println!("Activated!");
                enabled = true;
                active_idx = *active_iter.next().unwrap_or(&line.len());
            }
            if inactive_idx < idx {
                println!("Deactivated!");
                enabled = false;
                inactive_idx = *inactive_iter.next().unwrap_or(&line.len());
            }
            println!("{} {} {} {}", enabled, idx, active_idx, inactive_idx);
            if !enabled {
                continue;
            }
            // max three digit numbers, so we can optimize by looking
            // for the pattern in a short slice of idx..idx+12
            let max_end = std::cmp::min(idx+12, line.len());
            if let Some(end) = line[idx..max_end].find(")") {
                if line[idx..max_end].contains(",") {
                    let numbers = &mut line[idx+4..idx+end].split(",");
                    input.muls.push((
                        numbers.next().unwrap().parse::<i32>().unwrap(),
                        numbers.next().unwrap().parse::<i32>().unwrap(),
                    ));    
                }
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
        // println!("{} * {} = {}", a, b, a * b);
    }
    println!();
    println!("Total sum {}", input.sum);
}
