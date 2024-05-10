fn run() -> usize {
    const WIDTH: usize = 20;
    const HEIGHT: usize = 20;

    let mut paths: [[usize; WIDTH + 1]; HEIGHT + 1] = Default::default();
    for y in 0..=HEIGHT {
        for x in 0..=WIDTH {
            let mut path = 0;
            if x != 0 {
                path += paths[y][x - 1];
            }
            if y != 0 {
                path += paths[y - 1][x];
            }
            paths[y][x] = path.max(1);
        }
    }
    paths[HEIGHT][WIDTH]
}

fn main() {
    println!("{}", run());
}

#[test]
fn test() {
    assert_eq!(run(), 137846528820);
}
