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
    let gear = Regex::new(r"\*").unwrap();

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        numpos.extend(
            redig.captures_iter(&line).map(|c|
                (c.extract::<1>().0.to_string(), (index, c.get(0).unwrap().start())) // (num (str), (line, pos))
            ).collect::<Vec<(String, (usize, usize))>>()
        );

        symbols.extend(
            gear.captures_iter(&line).map(|c|
                (index, c.get(0).unwrap().start())
            ).collect::<Vec<(usize, usize)>>() // lines, pos
        );
    }

    let mut sum = 0;
    for (line, pos) in symbols {
        // not too proud, but want to keep uInt :)
        let rowmin = match line {
            0 => 0,
            _ => line-1
        };
        let colmin = match pos {
            0 => 0,
            _ => pos-1
        };

        let rows = (rowmin .. line + 2).collect::<Vec<usize>>();
        let cols = (colmin .. pos + 2).collect::<Vec<usize>>();

        let mut combinations: HashSet<(usize, usize)> = HashSet::new();
        for r in rows.iter() {
            for c in cols.iter() {
                combinations.insert((*r, *c));
            }
        }

        let mut candidates:Vec<&(String, (usize, usize))> = Vec::new();
        for item in numpos.iter(){
            let (num, coords) = item;

            // theese combs are to create set of all coords of numbers wrt to their length.
            // cant intersect anymore, because in case of number coords being withing starcombinations,
            // we dont know the numbers which we need for computing.
            let mut numcombs = HashSet::new();
            for col in coords.1 .. coords.1 + num.len()  {
                numcombs.insert((coords.0, col));
            }

            if numcombs.intersection(&combinations).count() > 0 {
                candidates.push(item);
            }
        }

        if candidates.len() == 2 {
            sum += candidates[0].0.parse::<usize>().unwrap() * candidates[1].0.parse::<usize>().unwrap();
        }
    }

    println!("sum: {}", sum);
}