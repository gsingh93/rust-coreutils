#![feature (macro_rules)]

use std::io;
use std::io::File;
use std::os;

mod util;

fn main() {
    let args = os::args();

    let mut num_lines = 1u;

    if args.len() == 1 {
        handle_stdin(&mut num_lines);
    }

    for arg in args.iter().skip(1) {
        let filename = arg.clone();
        if filename == ~"-" {
            handle_stdin(&mut num_lines);
        } else {
            let path = Path::new(filename);
            let mut file = try_result_or_fail!(File::open(&path));
            let text = try_result_or_fail!(file.read_to_str());
            for line in text.lines() {
                print_line(&mut num_lines, line);
            }
        }
    }
}

fn handle_stdin(num_lines: &mut uint) {
    for line in io::stdin().lines() {
        let line = try_result_or_fail!(line);
        let line = line.trim_right_chars('\n');
        print_line(num_lines, line);
    }
}

fn print_line(num_lines: &mut uint, line: &str) {
    if !line.is_empty() {
        println!("{} {}", *num_lines, line);
        *num_lines += 1;
    } else {
        println!("");
    }
}
