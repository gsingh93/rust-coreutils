#![feature (macro_rules)]

use std::io;
use std::io::{print, File};
use std::os;

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
            let path = Path::new(filename);
            let mut file = try_result_or_fail!(File::open(&path));
            let text = try_result_or_fail!(file.read_to_str());
            print(text);
        }
    }
}

fn handle_stdin() {
    for line in io::stdin().lines() {
        print(try_result_or_fail!(line));
    }
}
