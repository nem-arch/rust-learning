extern crate rand;
use std::cmp::Ordering;

fn main() {
    let mut min = 0;
    let mut max = 99;
    let mut cur = 50;
    let secret = rand::random::<u32>()%100u32;
    loop {
        println!("{} ?= {}", cur, secret);
        match cmp(cur, secret) {
            Ordering::Less => {
                min = cur;
                cur += (max - min)/2;
            }
            Ordering::Greater => {
                max = cur;
                cur -= (max - min)/2;
            }
            Ordering::Equal => {
                println!("i win!");
                break;
            }
        }
    }
}

fn cmp(a: u32, b: u32) -> Ordering {
    if a < b { Ordering::Less }
    else if a > b { Ordering::Greater }
    else { Ordering::Equal }
}
