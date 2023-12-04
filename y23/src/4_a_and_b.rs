use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::collections::HashSet;

fn getnums(strnums: &str) -> HashSet<u32> {
    let mut numbers: HashSet<u32> = HashSet::new();
    let re = Regex::new(r"(\d+)").unwrap();
    for (_, [digits]) in re.captures_iter(strnums).map(|c| c.extract()) {
        numbers.insert(digits.parse::<u32>().unwrap());
    }
    return numbers;
}

fn main() {
    let fpath = "src/4.txt";
    let file = File::open(fpath).unwrap();
    let reader = BufReader::new(file);

    let mut scores: Vec<u32> = Vec::new();

    // reading backwards would be the most elegant to get the initial scores.
    for line in reader.lines() {
        let line = line.unwrap();
        let mut sides = line.split("|").collect::<Vec<&str>>();
        sides[0] = &sides[0][sides[0].find(":").unwrap()..];
        assert_eq!((&sides).len(), 2usize);

        let winnums = getnums(sides[0]);
        let picknums = getnums(sides[1]);

        let overlap = winnums.intersection(&picknums).count() as u32;
        scores.push(overlap);
    }

    let taskA = scores.iter().map(|x| {
        if x > &0 {
            u32::pow(2, x - 1)
        } else { 0 }
    }).sum::<u32>();
    println!("Task A: {}", taskA);

    // enumerating reverse not so elegant
    for i in (0..scores.len()).rev() {
        let matches = scores[i];
        let mut cards = 0;

        // might need to take care of last elements, where matches are bigger than remaining cards
        for j in (i+1 ..i + 1 + matches  as usize) {
            if j >= scores.len() {
                break;
            }
            cards += scores[j] + 1;
        }
        scores[i] = cards;
    }
    let taskB: u32 = scores.iter().sum::<u32>() + scores.len() as u32;
    println!("Task B: {}", taskB);
}