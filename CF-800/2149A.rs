// Be Positive
// Version - 1.6

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t = read::<usize>(&mut it);

    for _ in 0..t {
        let n = read::<usize>(&mut it);

        let mut arr = Vec::with_capacity(n);
        for _ in 0..n {
            arr.push(read::<i64>(&mut it));
        }

        let mut operation = 0;
        let mut negative = 0;

        for &i in &arr {
            if i == 1 {
                continue;
            } else if i == 0 {
                operation += 1;
            } else if i == -1 {
                negative += 1;
            }
        }
        if negative % 2 == 1{
            operation += 2;
        }

        println!("{}", operation);
    }
}

fn read<T: std::str::FromStr>(it: &mut std::str::SplitWhitespace) -> T {
    it.next().unwrap().parse().ok().unwrap()
}
