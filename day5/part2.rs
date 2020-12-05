use util::*;

fn main() {
    let timer = Timer::new();
    let mut seats = input::line(&std::env::args().nth(1).unwrap())
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
        .collect::<Vec<u16>>();
    seats.sort();
    let mut last = seats[0] - 1;
    for s in seats {
        if s != last + 1 {
            break;
        }
        last = s;
    }
    timer.print();
    println!("{}", last + 1);
}
