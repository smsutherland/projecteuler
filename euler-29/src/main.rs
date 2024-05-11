use std::collections::HashSet;

use euler_util::factor_count;

const LIM: u64 = 100;

fn find_power(n: u64) -> (u64, u64) {
    let factors = factor_count(n);
    let max = factors.iter().map(|x| x.0).max().unwrap();
    for p in (2..=max).rev() {
        if factors.iter().map(|x| x.0 % p == 0).all(|x| x) {
            let new_factors = factors
                .into_iter()
                .map(|(old_p, factor)| (old_p / p, factor));
            let num = new_factors.fold(1, |acc, (pow, factor)| acc * factor.pow(pow as u32));
            return (num, p as u64);
        }
    }
    (n, 1)
}

fn run() -> usize {
    let mut items = HashSet::new();
    for a in 2..=LIM {
        let (base, pow) = find_power(a);
        for b in 2..=LIM {
            items.insert((base, b * pow));
        }
    }
    items.len()
}

fn main() {
    println!("{}", run());
}
