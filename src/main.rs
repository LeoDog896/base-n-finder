use rug::{Integer, ops::Pow};

fn main() {
    let mut count = 0;

    let prefix = Integer::from(8_675_309u32);
    let prefix_str = prefix.to_string();

    loop {
        let start = Integer::from(2).pow(count);
        if start.to_string().starts_with(&prefix_str) {
            println!("2^{count} starts with {prefix_str}");
            break;
        }

        println!("2^{count}");
        count += 1;
    }
}