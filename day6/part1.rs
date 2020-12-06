use util::*;

const LEN: usize = 'z' as usize - 'a' as usize + 1;

fn main() {
    let timer = Timer::new();
    let count: usize = input::vec::<String>(&std::env::args().nth(1).unwrap(), "\n\n")
        .iter()
        .map(|s| {
            let mut answered: [bool; LEN] = [false; LEN];
            for c in s.replace('\n', "").bytes() {
                answered[c as usize - 'a' as usize] = true;
            }
            answered.iter().filter(|&b| *b).count()
        })
        .sum();
    timer.print();
    println!("{}", count);
}
