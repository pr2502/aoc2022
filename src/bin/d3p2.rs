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
    let mut buf = [Vec::new(), Vec::new(), Vec::new()];
    let mut sum = 0u64;

    while stdin.read_until(b'\n', &mut buf[0]).unwrap() > 0 {
        stdin.read_until(b'\n', &mut buf[1]).unwrap();
        stdin.read_until(b'\n', &mut buf[2]).unwrap();

        let a = buf[0].as_slice().trim_ascii_end();
        let b = buf[1].as_slice().trim_ascii_end();
        let c = buf[2].as_slice().trim_ascii_end();

        for chr in a {
            if b.contains(chr) && c.contains(chr) {
                sum += priority(*chr);
                break;
            }
        }

        buf[0].clear();
        buf[1].clear();
        buf[2].clear();
    }

    println!("{sum}");
}
