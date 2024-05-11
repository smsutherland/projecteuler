fn run() -> u64 {
    const MAX: u64 = 1000;
    for sum in (0..=(2 * MAX - 2)).rev() {
        for a in ((sum.saturating_sub(MAX - 1).max(1))..sum.min(MAX)).rev() {
            let b = sum - a;
            let prod = a * b;
            let s = prod.to_string();
            if s == s.chars().rev().collect::<String>() {
                return prod;
            }
        }
    }
    panic!("Failed to find a palindrome product");
}

fn main() {
    println!("{}", run());
}
