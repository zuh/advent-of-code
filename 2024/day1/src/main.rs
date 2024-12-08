use std::fs;

struct Input {
    left: Vec<i32>,
    right: Vec<i32>,
}

fn read_input() -> Input {
    let mut input = Input {
        left: Vec::new(),
        right: Vec::new(),
    };
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let mut pieces = line.split_ascii_whitespace();
        input.left.push(pieces.next().unwrap().parse::<i32>().unwrap());
        input.right.push(pieces.last().unwrap().parse::<i32>().unwrap());
    }
    input
}

fn calc_distances(input: &Input) -> i32 {
    let mut distance = 0;
    for (left, right) in input.left.iter().zip(input.right.iter()) {
        distance += (left - right).abs()
    }
    distance
}

fn calc_similarity_scores(needles: &Vec<i32>, haystack: &Vec<i32>) -> i32 {
    let mut score = 0;
    for needle in needles {
        let mut needle_count = 0;
        for hay in haystack {
            if needle == hay {
                needle_count += 1;
            }
        }
        score += needle * needle_count;
    }
    score
}

fn main() {
    let mut input = read_input();
    input.left.sort();
    input.right.sort();
    let distance = calc_distances(&input);
    let left_similar = calc_similarity_scores(&input.left, &input.right);
    let right_similar = calc_similarity_scores(&input.right, &input.left);
    let zipbag = input.left.iter().zip(input.right.iter());
    for (&left, &right) in zipbag {
        println!("{} => {} == {} {} {}",
            left, right, distance, left_similar, right_similar)
    }
}

#[test]
fn test_main() {
    main()
}
