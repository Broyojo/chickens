use rayon::prelude::*;

fn main() {
    let total = 10u64.pow(10) - 1;

    let successes = (0..=total).into_par_iter().map(check).sum::<u64>();

    println!("successes: {}, total: {}", successes, total);
    println!("prob: {}", successes as f64 / total as f64);
}

fn check(n: u64) -> u64 {
    let s = format!("{:0>10}", n);

    let mut counts = [0; 10];

    for c in s.chars() {
        counts[c.to_digit(10).unwrap() as usize] += 1;
    }

    let mut alive = 0;

    for count in counts {
        if count == 0 {
            alive += 1;
        }
    }

    if alive == 1 {
        //println!("{} {:?}", s, counts);
        1
    } else {
        0
    }
}
