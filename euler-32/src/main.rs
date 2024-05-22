use std::collections::HashSet;

use itertools::Itertools;

fn run() -> u64 {
    let chars = "123456789  ";
    let mut founds = HashSet::new();
    for perm in chars.chars().permutations(chars.len()) {
        let perm = String::from_iter(perm);
        let nums: Vec<u64> = match perm.split(' ').map(|s| s.parse()).collect() {
            Ok(n) => n,
            Err(_) => continue,
        };
        if nums[0] * nums[1] == nums[2] {
            founds.insert(nums[2]);
        }
    }

    founds.into_iter().sum()
}

fn main() {
    println!("{}", run());
}
