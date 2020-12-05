use util::*;

fn main() {
    let timer = Timer::new();
    let max = input::line(&std::env::args().nth(1).unwrap())
        .split('\n')
        .map(|s| {
            s.chars().enumerate().fold(0, |a, (n, c)| {
                if c == 'B' || c == 'R' {
                    a | 1 << (s.len() - 1 - n)
                } else {
                    a
                }
            })
        })
        .max()
        .unwrap();
    timer.print();
    println!("{}", max);
}
