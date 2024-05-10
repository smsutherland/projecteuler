use euler_util::factor_count;

fn run() -> u64 {
    let mut triangle = 0;
    let mut n = 1;
    loop {
        triangle += n;
        n += 1;
        let factors = factor_count(triangle);
        let divisors: u64 = factors.into_iter().map(|x| x.0 as u64 + 1).product();
        if divisors > 500 {
            break;
        }
    }
    triangle
}

fn main() {
    println!("{}", run());
}

#[test]
fn euler_12() {
    assert_eq!(run(), 76576500);
}
