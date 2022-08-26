use radix_fmt::radix;
use rayon::prelude::*;

fn main() {
    let num_alive = 1;
    for num_chickens in 2..=10 {
        let total = (num_chickens as u64).pow(num_chickens as u32);
        let successes = (0..total)
            .into_par_iter()
            .map(|n: u64| -> u64 {
                let s = pad_zeros_left(radix(n, num_chickens).to_string(), num_chickens as usize);

                //println!("{}", s);

                let mut counts = vec![0; num_chickens as usize];

                for c in s.chars() {
                    counts[c.to_digit(num_chickens as u32).unwrap() as usize] += 1;
                }

                let alive = counts.iter().filter(|c| **c == 0).count();

                if alive == num_alive {
                    //println!("{} {:?}", s, counts);
                    1
                } else {
                    0
                }
            })
            .sum::<u64>();

        let n_gcd = gcd(successes, total);

        println!(
            "test with {} chickens: {} successes out of {} total runs: prob = {}/{} ≈ {} ",
            num_chickens,
            successes,
            total,
            successes / n_gcd,
            total / n_gcd,
            successes as f64 / total as f64
        );
    }
}

fn pad_zeros_left(s: String, n: usize) -> String {
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

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        a %= b;
        if a == 0 {
            return b;
        }
        b %= a;
    }
    a
}
