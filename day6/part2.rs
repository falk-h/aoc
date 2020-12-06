use util::*;

const LEN: usize = 'z' as usize - 'a' as usize + 1;

fn main() {
    let timer = Timer::new();
    let count: usize = input::vec::<String>(&std::env::args().nth(1).unwrap(), "\n\n")
        .iter()
        .map(|s| {
            let answers: Vec<[bool; LEN]> = s
                .split('\n')
                .map(|s| {
                    let mut answered: [bool; LEN] = [false; LEN];
                    for c in s.bytes() {
                        answered[c as usize - 'a' as usize] = true;
                    }
                    answered
                })
                .collect();
            let mut count = 0;
            for i in 0..LEN {
                let mut in_all = true;
                for j in 0..answers.len() {
                    in_all &= answers[j][i];
                }
                if in_all {
                    count += 1;
                }
            }
            count
        })
        .sum();
    timer.print();
    println!("{}", count);
}
