fn run() -> u64 {
    let mut sum: u64 = 0;
    let mut a = 1;
    let mut b = 1;
    loop {
        if b > 4000000 {
            break;
        }
        let c = a + b;
        if c % 2 == 0 {
            sum += c;
        }
        a = b;
        b = c;
    }
    sum
}

fn main() {
    println!("{}", run());
}

#[test]
fn euler_2() {
    assert_eq!(run(), 4613732);
}
