use regex::Regex;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

fn getround(r: &str) -> HashMap<&str, u8> {
    let mut counts: HashMap<&str, u8> = HashMap::new();
    let re = Regex::new(r"(\d+) (\w+)").unwrap();
    for (_, [count, color]) in re.captures_iter(r).map(|c| c.extract()) {
        // hoping there are no issues
        counts.insert(color, count.parse::<u8>().unwrap());
        // println!("count: {} -> {}", count, count.parse::<u8>().unwrap());
    }
    return counts;
}

fn checkround(counts: &HashMap<&str, u8>, colors: &HashMap<&str, u8>) -> bool {
    let mut constraints = true;
    for (color, count) in counts {
        constraints &= colors.get(color).unwrap_or(&0) >= count;
    }
    return constraints;
}

fn checkgame(game: &str, colors: &HashMap<&str, u8>) -> u32 {
    let i = "Game ".len();
    let gameid: u32 = game[i..game.find(":").unwrap()].parse::<u32>().unwrap();

    let mut constraints = true;
    for round in game.split(";") {
        constraints &= checkround(&getround(round), colors);
    }

    return gameid * constraints as u32; 
}

fn main() {
    let colortuples =  [("red", 12), ("green", 13), ("blue", 14)];
    let colors: HashMap<&str, u8> = colortuples.into_iter().collect();

    let fpath = "src/2.txt";
    let file = File::open(fpath).unwrap();
    
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;
    for line in reader.lines() {
        sum += checkgame(&line.unwrap(), &colors);
    }
    println!("Sum: {}", sum);

}