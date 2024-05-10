use euler_util::divisors;

const LIMIT: u64 = 28123;

fn find_abundant() -> Vec<u64> {
    let mut res = Vec::new();
    for i in 1..=LIMIT {
        let div = divisors(i);
        if div.into_iter().sum::<u64>() > i {
            res.push(i);
        }
    }
    res
}

fn run() -> u64 {
    let abundant_nums = find_abundant();
    let mut abundant_sums: Vec<_> = abundant_nums
        .iter()
        .copied()
        .flat_map(|num1| abundant_nums.iter().copied().map(move |num2| num1 + num2))
        .collect();
    abundant_sums.sort_unstable();
    let mut sum = 0;
    for i in 1..=LIMIT {
        if abundant_sums.binary_search(&i).is_err() {
            sum += i;
        }
    }
    sum
}

fn main() {
    println!("{}", run());
}

#[test]
fn euler_23() {
    assert_eq!(run(), 4179871);
}
