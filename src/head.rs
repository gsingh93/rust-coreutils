#![feature (macro_rules)]

use std::io;
use std::io::{BufferedReader, File, print};
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
            let p = Path::new(filename);
            let mut br = BufferedReader::new(try_result_or_fail!(File::open(&p)));
            for line in br.lines().take(10) {
                print!("{}", line.unwrap());
            }
        }
    }
}

fn handle_stdin() {
    for line in io::stdin().lines().take(10) {
        print(try_result_or_fail!(line));
    }
}
