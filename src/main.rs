use rug::{Integer, ops::Pow};

fn main() {
    let mut start = Integer::from(2u32);
    let mut count = 0;

    let prefix = Integer::from(8675309u32);
    let prefix_str = prefix.to_string();

    loop {
        if start.to_string().starts_with(&prefix_str) {
            println!("2^{} starts with {}", count, prefix_str);
            break;
        }

        println!("2^{}", count);
        start = start.pow(2);
        count += 1;
    }
}