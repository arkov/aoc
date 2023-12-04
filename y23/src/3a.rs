use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::{Regex};
use std::collections::HashSet;

fn main() {
    let fpath = "src/3.txt";
    let file = File::open(fpath).unwrap();
    let reader = BufReader::new(file);

    let mut numpos: HashSet<(String, (usize, usize))> = HashSet::new(); // (num, (line, pos)) ..., len))
    let mut symbols: HashSet<(usize, usize)> = HashSet::new();

    let redig = Regex::new(r"(\d+)").unwrap();
    let resym = Regex::new(r"[^0-9.\s]").unwrap();

    for (index, line) in reader.lines().enumerate() { ;
        let line = line.unwrap();
         numpos.extend(
             redig.captures_iter(&line).map(|c|
                 (c.extract::<1>().0.to_string(), (index, c.get(0).unwrap().start()))
             ).collect::<Vec<(String, (usize, usize))>>()
         );

        symbols.extend(
            resym.captures_iter(&line).map(|c|
                (index, c.get(0).unwrap().start())
            ).collect::<Vec<(usize, usize)>>()
        );
    }

    let mut sum = 0;
    for (num, (line, pos)) in numpos {
        let rowmin = match line {
            0 => 0,
            _ => line-1
        };
        let colmin = match pos {
            0 => 0,
            _ => pos-1
        };

        let rows = (rowmin .. line+2).collect::<Vec<usize>>();
        let cols = (colmin .. pos + num.len() + 1).collect::<Vec<usize>>();

        let mut combinations: HashSet<(usize, usize)> = HashSet::new();
        for r in rows.iter() {
            for c in cols.iter() {
                combinations.insert((*r, *c));
            }
        }

        if combinations.intersection(&symbols).count() > 0 {
            sum += num.parse::<usize>().unwrap();
        }

    }

    println!("sum: {}", sum);
}