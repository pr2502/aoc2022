use std::io::{self, BufRead};
use Out::*;
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
            b'A' => Rock,
            b'B' => Paper,
            b'C' => Scisors,
            _ => unreachable!("invalid input"),
        }
    }
}

#[derive(Clone, Copy)]
enum Out {
    Win,
    Draw,
    Loss,
}

impl Out {
    fn from_byte(byte: u8) -> Out {
        match byte {
            b'X' => Loss,
            b'Y' => Draw,
            b'Z' => Win,
            _ => unreachable!("invalid input"),
        }
    }
}

fn main() {
    let mut stdin = io::stdin().lock();
    let mut buf = Vec::new();
    let mut score = 0u64;

    while stdin.read_until(b'\n', &mut buf).unwrap() > 0 {
        let &[op, _, out, _] = buf.as_slice() else { panic!("invalid input"); };
        let (op, out) = (RPS::from_byte(op), Out::from_byte(out));
        buf.clear();

        let win = match out {
            Loss => 0,
            Draw => 3,
            Win => 6,
        };

        let me = match (out, op) {
            (Draw, Rock) | (Loss, Paper) | (Win, Scisors) => Rock,
            (Draw, Paper) | (Loss, Scisors) | (Win, Rock) => Paper,
            (Draw, Scisors) | (Loss, Rock) | (Win, Paper) => Scisors,
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
