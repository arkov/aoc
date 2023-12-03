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

// the counts lifetime is afaik not needed, but removing it causes errors.
// as we already have all the keys in minset and .insert only updates values, we should not be borrowing from counts.
// but rust complains about lifetimes, so whatever.
// if you know something about this, please let me know. i would immensly appreciate it.
fn checkround<'a>(counts: &HashMap<&'a str, u8>, colors: &HashMap<&'a str, u8>) -> HashMap<&'a str, u8> {
    let mut minset: HashMap<&'a str, u8> = colors.iter().map(|(k, _)| (*k, 0)).collect();

    for (color, count) in counts {
        // if minset.contains_key(color) && minset.get(color).unwrap_or(&0) < count {
        //     minset.insert(color, *count);
        // }
        if minset.get(color).unwrap_or(&0) < count {
            minset.insert(color, *count); 
            // not updating keys, thus never should be borrowing from counts, but it stillcomplains about lifetime
        }
    }
    return minset;
}

fn checkgame<'a>(game: &str, colors: &HashMap<&'a str, u8>) -> u32 {
    let mut minset: HashMap<&str, u8> = colors.iter().map(|(k, _)| (*k, 0)).collect::<HashMap<&'a str, u8>>();

    for round in game.split(";") {
        checkround(&getround(round), &minset).iter().for_each(|(k, v)| {
            if minset.get(k).unwrap_or(&0) < v {
                minset.insert(k, *v);
            }
        });
    }
    let sum: u32 = minset.iter().fold(1, |acc, (_, v)| acc * (*v as u32));
    // println!("{} -> {:?} is {}", game, minset, sum);
    return sum;
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