// Candies for Nephews
// Version 1.2

use std::io::{self, Read};

fn main() {

    const NEPHEWS: usize = 3;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t = read::<usize>(&mut it);

    for _ in 0..t {
        let n = read::<usize>(&mut it);
        let mut output: usize = 0;

        // Negative Remainders
        if n % 3 != 0 {
            output = NEPHEWS - (n % 3);
        }
        println!("{}", output);
    }
}

fn read<T: std::str::FromStr>(it: &mut std::str::SplitWhitespace) -> T {
    it.next().unwrap().parse().ok().unwrap()
}
