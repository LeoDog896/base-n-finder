use rug::Integer;
use std::io::Write;

fn main() {
    let mut start = Integer::from(2);
    let mut count = 0;

    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    let prefix = Integer::from(8_675_309u32);
    let prefix_str = prefix.to_string();

    loop {
        start = start * 2;
        let string = start.to_string_radix(10);
        if string.starts_with(&prefix_str) {
            let _ = writeln!(lock, "2^{count} starts with {prefix_str}");
            break;
        }

        let _ = writeln!(lock, "2^{count}");
        count += 1;
    }
}