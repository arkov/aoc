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
    let fpath = "src/3.txt";
    let file = File::open(fpath).unwrap();
    let reader = BufReader::new(file);

    let mut sum: u32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut sides = line.split("|").collect::<Vec<&str>>();
        sides[0] = &sides[0][sides[0].find(":").unwrap()..];
        assert_eq!((&sides).len(), 2usize);

        let winnums = getnums(sides[0]);
        let picknums = getnums(sides[1]);

        let overlap = winnums.intersection(&picknums).count() as u32;
        if overlap <= 0 {
            continue;
        }
        sum += u32::pow(2, overlap - 1);
    }
    println!("Sum: {}", sum);
}