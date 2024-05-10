use std::num::NonZeroUsize;

fn run() -> usize {
    const MAX: usize = 1_000_000;

    let mut paths: Vec<Option<NonZeroUsize>> = vec![None; MAX];
    paths[0] = NonZeroUsize::new(1);
    for n in 2..=MAX {
        match paths[n - 1] {
            Some(_) => continue,
            None => {
                let mut visits = vec![n];
                loop {
                    let last = *visits.last().unwrap();
                    let next = if last % 2 == 0 {
                        last / 2
                    } else {
                        3 * last + 1
                    };
                    if let Some(Some(x)) = paths.get(next - 1) {
                        let x = x.get();
                        for (i, n) in visits.into_iter().rev().enumerate() {
                            if n <= MAX {
                                paths[n - 1] = NonZeroUsize::new(x + i + 1);
                            }
                        }
                        break;
                    } else {
                        visits.push(next);
                    }
                }
            }
        }
    }

    let mut max = 0;
    let mut num = 0;
    for (i, path) in paths[..MAX].iter().enumerate() {
        let path = path.unwrap().get();
        if max < path {
            max = path;
            num = i + 1;
        }
    }
    num
}

fn main() {
    println!("{}", run());
}

#[test]
fn euler_15() {
    assert_eq!(run(), 837799);
}
