use std::io;
use std::rand;
use std::cmp::Ordering;

fn main() {
    let secret = rand::random::<u32>()%100u32;
    loop {
        println!("guess:");
        let input = io::stdin().read_line()
                                .ok()
                                .expect("failed reading");
        let input_num: Option<u32> = input.trim().parse();
        let num = match input_num {
            Some(num) => num,
            None      => {
                println!("not a num");
                continue;
            }
        };
        match cmp(num, secret) {
            Ordering::Less    => println!("bigger"),
            Ordering::Greater => println!("smaller"),
            Ordering::Equal   => {
                println!("win!");
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
