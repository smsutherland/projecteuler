fn run() -> u64 {
    let factorials = {
        let mut digits = [1, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        for i in 1..10 {
            digits[i] *= digits[i - 1];
        }
        digits
    };

    let form_nums = |digits: &[usize]| {
        let fact_sum: u64 = digits.iter().copied().map(|d| factorials[d]).sum();
        let num: u64 = digits
            .iter()
            .copied()
            .enumerate()
            .map(|(i, d)| (10u64).pow(i as u32) * d as u64)
            .sum();

        (fact_sum, num)
    };

    let mut sum = 0;

    let mut digits = vec![0, 1];
    let mut digit = 0;
    loop {
        let (fact_num, actual_num) = form_nums(&digits);
        if digit == 0 && actual_num > fact_num + factorials[9] {
            break;
        }
        if fact_num == actual_num {
            sum += actual_num;
        }
        if fact_num > actual_num {
            for d in digits.iter_mut().take(digit + 1) {
                *d = 0;
            }
            digit += 1;
        }
        loop {
            if digits.len() == digit {
                digits.push(0);
            }
            if digits[digit] == 9 {
                digits[digit] = 0;
                digit += 1;
            } else {
                digits[digit] += 1;
                digit = 0;
                break;
            }
        }
    }

    sum
}

fn main() {
    println!("{}", run());
}
