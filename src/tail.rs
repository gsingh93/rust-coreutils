#![feature (macro_rules)]

extern crate collections;

use std::io;
use std::io::{BufferedReader, File};
use std::os;

use collections::deque::Deque;
use collections::ringbuf::RingBuf;

mod util;

fn main() {
    let args = os::args();

    if args.len() == 1 {
        handle_stdin();
    }

    for arg in args.iter().skip(1) {
        let filename = arg.clone();
        if filename == ~"-" {
            handle_stdin();
        } else {
            let p = Path::new(filename);
            let mut br_fast = BufferedReader::new(try_result_or_fail!(File::open(&p)));
            let mut br_slow = BufferedReader::new(try_result_or_fail!(File::open(&p)));
            let mut it_slow = br_slow.lines();
            let mut i = 0;
            for _ in br_fast.lines() {
                if i >= 10 {
                    it_slow.next();
                }
                i += 1;
            }

            for line in it_slow {
                print!("{}", line.unwrap());
            }
        }
    }
}

fn handle_stdin() {
    let mut last_ten_lines: RingBuf<~str> = RingBuf::new();
    for line in io::stdin().lines() {
        last_ten_lines.push_back(line.unwrap());
        if last_ten_lines.len() > 10 {
            last_ten_lines.pop_front();
        }
    }

    for line in last_ten_lines.iter() {
        print!("{}", line);
    }
}
