const NUM: u64 = 1_000_000 - 1;
const DIGITS: u64 = 10;

fn factorial(n: u64) -> u64 {
    let mut res = 1;
    for i in 1..=n {
        res *= i;
    }
    res
}

fn run() -> u64 {
    let mut remainder = NUM;
    let mut number = 0;
    let mut used_digits = [false; 10];
    for i in (0..DIGITS).rev() {
        let fact = factorial(i);
        let digit = remainder / fact;
        remainder -= digit * fact;
        let digit = used_digits
            .iter()
            .enumerate()
            .filter(|x| !x.1)
            .nth(digit as usize)
            .unwrap()
            .0;
        used_digits[digit] = true;
        number *= 10;
        number += digit as u64;
    }
    number
}

fn main() {
    println!("{:<03}", run());
}
