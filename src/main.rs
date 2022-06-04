use radix_fmt::radix;
use rayon::prelude::*;

const NUM_CHICKENS: u32 = 10;

fn main() {
    let total = (NUM_CHICKENS as u64).pow(NUM_CHICKENS);

    println!("total: {}", total);

    let successes = (0..total).into_par_iter().map(check).sum::<u64>();

    println!("successes: {}, total: {}", successes, total);
    println!("prob: {}", successes as f64 / total as f64);
}

fn check(n: u64) -> u64 {
    let s = pad_space_left(
        radix(n, NUM_CHICKENS as u8).to_string(),
        NUM_CHICKENS as usize,
    );

    //println!("{}", s);

    let mut counts = [0; NUM_CHICKENS as usize];

    for c in s.chars() {
        counts[c.to_digit(NUM_CHICKENS).unwrap() as usize] += 1;
    }

    let mut alive = 0;

    for count in counts {
        if count == 0 {
            alive += 1;
        }
    }

    //println!("{} {:?}", s, counts);

    if alive == 1 {
        1
    } else {
        0
    }
}

fn pad_space_left(s: String, n: usize) -> String {
    if s.len() > n {
        return s;
    }

    let num_pad = n - s.len();

    let mut padded = String::with_capacity(s.len() + num_pad);

    for _ in 0..num_pad {
        padded.push('0');
    }

    padded.push_str(&s);

    padded
}
