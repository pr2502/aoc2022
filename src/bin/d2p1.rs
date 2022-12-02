use std::io::{self, BufRead};
use RPS::*;

#[derive(Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scisors,
}

impl RPS {
    fn from_byte(byte: u8) -> RPS {
        match byte {
            b'A' | b'X' => Rock,
            b'B' | b'Y' => Paper,
            b'C' | b'Z' => Scisors,
            _ => unreachable!("invalid input"),
        }
    }
}

fn main() {
    let mut stdin = io::stdin().lock();
    let mut buf = Vec::new();
    let mut score = 0u64;

    while stdin.read_until(b'\n', &mut buf).unwrap() > 0 {
        let &[op, _, me, _] = buf.as_slice() else { panic!("invalid input"); };
        let (op, me) = (RPS::from_byte(op), RPS::from_byte(me));
        buf.clear();

        let win = match (me, op) {
            (Rock, Scisors) | (Paper, Rock) | (Scisors, Paper) => 6, // win
            (Rock, Rock) | (Paper, Paper) | (Scisors, Scisors) => 3, // draw
            (Rock, Paper) | (Paper, Scisors) | (Scisors, Rock) => 0, // loss
        };

        let shape = match me {
            Rock => 1,
            Paper => 2,
            Scisors => 3,
        };

        score += win + shape;
    }

    println!("{score}");
}
