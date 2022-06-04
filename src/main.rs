use rayon::prelude::*;

fn main() {
    let total = 10u64.pow(10) - 1;
    let offset = 10u64.pow(9);

    let successes = (offset..=total).into_par_iter().map(check).sum::<u64>();

    println!("successes: {}, total: {}", successes, total);
    println!("prob: {}", successes as f64 / total as f64);
}

fn check(n: u64) -> u64 {
    let s = format!("{:0>10}", n);

    let mut alive = 0;

    for c in s.chars() {
        if c.to_digit(10).unwrap() == 0 {
            alive += 1;
        }
    }

    if alive == 1 {
        1
    } else {
        0
    }
}
