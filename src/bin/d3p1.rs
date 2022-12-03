#![feature(byte_slice_trim_ascii)]

use std::io::{self, BufRead};

fn priority(byte: u8) -> u64 {
    match byte {
        b'a'..=b'z' => 1 + (byte - b'a') as u64,
        b'A'..=b'Z' => 27 + (byte - b'A') as u64,
        _ => unreachable!("invalid input"),
    }
}

fn main() {
    let mut stdin = io::stdin().lock();
    let mut buf = Vec::new();
    let mut sum = 0u64;

    while stdin.read_until(b'\n', &mut buf).unwrap() > 0 {
        let contents = buf.as_slice().trim_ascii_end();
        let (first, second) = contents.split_at(contents.len() / 2);

        for chr in first {
            if second.contains(chr) {
                sum += priority(*chr);
                break;
            }
        }

        buf.clear();
    }

    println!("{sum}");
}
