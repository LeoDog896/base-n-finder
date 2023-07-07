use rug::{ops::Pow, Integer};
use std::{env::args, io::Write};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

// highest so far has been 2^2333833
fn main() {
    let mut count = args()
        .nth(1)
        .unwrap_or_else(|| "0".to_string())
        .parse::<u32>()
        .unwrap();
    let mut start = Integer::from(2).pow(count);

    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    let prefix = Integer::from(8_675_309u32);
    let prefix_str = prefix.to_string();

    loop {
        start *= 2;
        let string = start.to_string_radix(10);
        if string.starts_with(&prefix_str) {
            let _ = writeln!(lock, "2^{count} starts with {prefix_str}");
            break;
        }

        let _ = writeln!(lock, "2^{count}");
        count += 1;
    }
}
