#![feature(macro_rules)]

use std::os;
use std::path::Path;
use std::io::File;

mod util;

fn main() {
    let args = os::args();

    for filename in args.iter().skip(1) {
        let path = Path::new(filename.clone());
        let mut file = try_result_or_fail!(File::open(&path));
        let text = try_result_or_fail!(file.read_to_str());

        let lines = count_lines(text);
        let words = count_words(text);
        let chars = count_chars(text);

        println!("{} {} {} {}", lines, words, chars, filename);
    }
}

fn count_chars(text: &str) -> uint {
    text.len()
}

fn count_words(text: &str) -> uint {
    text.words().len()
}

fn count_lines(text: &str) -> uint {
    text.lines().len()
}
