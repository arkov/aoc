use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
use std::cmp::{min, max};

fn main() {
    let fpath = "src/1.txt";
    let file = File::open(fpath).unwrap();
    
    let reader = BufReader::new(file);
    let mut sum: u64 = 0;


    for line in reader.lines() {
        let l = line.unwrap().chars().collect::<Vec<char>>();
        // println!("Line: {:?}", l);

        let mut a: u8 = 10;
        let mut b: u8 = 10;

        let numstr = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        let numint: [u8; 9]= [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let numdict: HashMap<&str, &u8> = numstr.iter().copied().zip(numint.iter()).collect::<HashMap<&str, &u8>>();

        // could implement simple DFA with lookahead, only testing for specific subet of possibilities (t... -> {two, three}), but this is simpler.
        for i in 0..l.len() {
            // either numeric or alphabetic (further down)
            if l[i].is_ascii_digit() {
                match a {
                    10 => a = l[i].to_digit(10).unwrap() as u8,
                    _ => b = l[i].to_digit(10).unwrap() as u8,
                }
                continue;
            }

            let mut possible = numstr.to_vec();
            let mut j = 0;
            while possible.len() > 1 && (i+j) < l.len() && l[i+j].is_ascii_alphabetic() {
                let slice = &l[i..min(i+j+1, l.len())].iter().collect::<String>();
                possible = possible.iter().filter(|a| a.starts_with(slice)).copied().collect::<Vec<&str>>();
                j += 1;
            }
            if possible.len() == 1 && numdict.contains_key(&*(l[i..min(i+possible[0].len(), l.len())].iter().collect::<String>())) { // garbage
                let matched: &u8 = numdict[possible[0]]; // garbage
                // repeated code, should be a function
                if a >= 10 {
                    a = *matched;
                } else {
                    b = *matched;
                }
            }
        }
        if a >= 10 {
            println!("Invalid line: {}", l.iter().collect::<String>());
            continue;
        }
        if b >= 10 {
            b = a;
        }
        // println!("For l: {:?} -- {}**10 + {} = {}", l, a, b, a*10 + b);
        sum += (a*10 + b) as u64;// + b;
    }
    println!("Sum: {}", sum);


}