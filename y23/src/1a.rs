use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let fpath = "src/1.txt";
    let file = File::open(fpath).unwrap();
    
    let reader = BufReader::new(file);
    let mut sum: u64 = 0;

    for line in reader.lines() {
        let l = line.unwrap();
        
        let mut a: u8 = 10;
        let mut b: u8 = 10;

        for c in l.chars() {
            if c.is_ascii_digit() && a >= 10{
                a = c.to_digit(10).unwrap() as u8;
            } else if c.is_ascii_digit() {
                b = c.to_digit(10).unwrap() as u8;
            }
        }
        if a >=10 {
            println!("Invalid line: {}", l);
            continue;
        }
        if b >= 10 {
            b = a;
        }
        // println!("{}**10 + {} = {}", a, b, a*10 + b);
        sum += (a*10 + b) as u64;// + b;
    }
    println!("Sum: {}", sum);


}