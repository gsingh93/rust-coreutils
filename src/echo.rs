use std::io::println;
use std::os;

fn main() {
    let args = os::args();
    let mut result = StrBuf::new();

    for i in range(1, args.len()) {
        result.push_str(args[i]);
        if i != args.len() - 1 {
            result.push_str(" ");
        }
    }

    println(result.into_owned());
}
