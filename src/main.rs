use num_bigint::BigUint;

fn main() {
    let mut start = BigUint::from(2u32);
    let mut count = 0;

    loop {
        if start.to_string().starts_with("8675309") {
            println!("2^{} starts with 8675309", count);
            break;
        }

        println!("2^{}", count);
        start = start.clone() * start;
        count += 1;
    }
}