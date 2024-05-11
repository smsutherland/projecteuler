const POW: u32 = 5;

fn log10(mut num: u64) -> u64 {
    let mut i = 0;
    while num != 0 {
        num /= 10;
        i += 1;
    }
    i
}

fn digits(mut num: u64) -> Vec<u8> {
    let mut res = Vec::new();
    loop {
        res.push((num % 10) as u8);
        num /= 10;
        if num == 0 {
            return res;
        }
    }
}

fn run() -> u64 {
    // find max number
    let nine = 9u64.pow(POW);
    let mut digit_count = log10(nine);
    while log10(digit_count * nine) != digit_count {
        digit_count += 1;
    }
    let max = nine * digit_count;
    let mut sum = 0;
    for i in 2..=max {
        let pow_sum: u64 = digits(i).into_iter().map(|x| (x as u64).pow(POW)).sum();
        if pow_sum == i {
            sum += i;
        }
    }
    sum
}

fn main() {
    println!("{}", run());
}
