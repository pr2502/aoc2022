use std::io::{self, BufRead};

fn main() {
    let mut counter = 0u64;

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let line = line.trim();

        let (a, b) = line.split_once(',').unwrap();
        let (a1, a2) = a.split_once('-').unwrap();
        let (b1, b2) = b.split_once('-').unwrap();

        let [a1, a2, b1, b2]: [u64; 4] = [
            a1.parse().unwrap(),
            a2.parse().unwrap(),
            b1.parse().unwrap(),
            b2.parse().unwrap(),
        ];

        if (a1..=a2).contains(&b1)
            || (a1..=a2).contains(&b2)
            || (b1..=b2).contains(&a1)
            || (b1..=b2).contains(&a2)
        {
            counter += 1;
        }
    }

    println!("{counter}");
}
