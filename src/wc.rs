#![feature(macro_rules)]

use std::os;
use std::path::Path;
use std::io::File;

macro_rules! try_and_fail(
    ($e:expr) => (match $e { Ok(e) => e, Err(e) => fail!(e.desc) });
)

fn main() {
    let args = os::args();

    for filename in args.iter().skip(1) {
        let path = Path::new(filename.clone());
        let mut file = try_and_fail!(File::open(&path));
        let text = try_and_fail!(file.read_to_str());

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
