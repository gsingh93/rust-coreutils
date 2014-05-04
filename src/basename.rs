#![feature(macro_rules)]

use std::os;
use std::io::println;

fn main() {
    let args = os::args();
    if args.len() == 1 {
        fail!("{}: missing operand", args[0]);
    } else if args.len() > 3 {
        fail!("{}: extra operand {}", args[0], args[2]);
    }

    let name = args[1].clone();
    let mut suffix: Option<~str> = None;
    if args.len() == 3 {
        suffix = Some(args[2]);
    }

    println(basename(name, suffix));
}

fn basename(name: &str, suffix: Option<~str>) -> ~str {
    // Handle trailing slashes
    let name = name.trim_right_chars('/');
    if name.is_empty() {
        return ~"/";
    }

    let index = match name.rfind('/') {
        Some(index) => index,
        None => -1
    };

    let mut bname =
        if index == -1 {
            name
        } else {
            name.slice_from(index + 1)
        };

    if suffix.is_some() {
        let suffix = suffix.unwrap();
        if bname.ends_with(suffix) {
            bname = bname.slice_to(bname.len() - suffix.len());
        }
    }
    bname.to_owned()
}
