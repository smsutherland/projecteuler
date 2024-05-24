use euler_util::rational::Rational;

fn digits(n: u64) -> [u8; 2] {
    [(n / 10) as u8, (n % 10) as u8]
}

fn cancel_digits(numer: [u8; 2], denom: [u8; 2]) -> Option<(u8, u8)> {
    if numer[0] == denom[0] {
        Some((numer[1], denom[1]))
    } else if numer[1] == denom[1] {
        Some((numer[0], denom[0]))
    } else if numer[0] == denom[1] {
        if numer[1] == denom[0] {
            Some((1, 1))
        } else {
            Some((numer[1], denom[0]))
        }
    } else if numer[1] == denom[0] {
        if numer[0] == denom[1] {
            Some((1, 1))
        } else {
            Some((numer[0], denom[1]))
        }
    } else {
        None
    }
}

fn run() -> u64 {
    let mut product = Rational::new(1, 1);

    for numerator in 10..100 {
        let numer_digits = digits(numerator);
        if numer_digits[1] == 0 {
            continue;
        }

        for denominator in (numerator + 1)..100 {
            if denominator == numerator {
                continue;
            }
            let denom_digits = digits(denominator);
            let Some((numer, denom)) = cancel_digits(numer_digits, denom_digits) else {
                continue;
            };
            if denom == 0 {
                continue;
            }

            let actual = Rational::new(numerator, denominator);
            let weird = Rational::new(numer as u64, denom as u64);
            if actual == weird {
                product *= actual;
                product.reduce();
            }
        }
    }

    product.denominator
}

fn main() {
    println!("{}", run());
}
