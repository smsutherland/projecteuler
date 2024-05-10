fn run() -> u64 {
    (1..1000)
        .filter(|x| (x % 3 == 0) | (x % 5 == 0))
        .sum::<u64>()
}

fn main() {
    println!("{}", run());
}

#[test]
fn euler_1() {
    assert_eq!(run(), 233168);
}
