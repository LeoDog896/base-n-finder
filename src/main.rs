use rug::{Integer, ops::Pow};
use std::io::Write;

fn main() {
    let mut count = 0;

    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    let prefix = Integer::from(8_675_309u32);
    let prefix_str = prefix.to_string();

    loop {
        let start = Integer::from(2).pow(count);
        if start.to_string().starts_with(&prefix_str) {
            let _ = writeln!(lock, "2^{count} starts with {prefix_str}");
            break;
        }

        let _ = writeln!(lock, "2^{count}");
        count += 1;
    }
}