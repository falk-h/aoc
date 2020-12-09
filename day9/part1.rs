use util::*;

const LEN: usize = 25;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timer = Timer::new();
    let input = input::lines::<usize>(&std::env::args().nth(1).unwrap());
    let mut lookup = vec![[0; LEN - 1]; input.len()];

    for (i, n) in input.iter().enumerate() {
        let start = if i < LEN { 0 } else { i + 1 - LEN };
        for (j, m) in input[start..i].iter().enumerate() {
            lookup[i][j] = n + m;
        }
    }

    timer.print();
    let timer = Timer::new();

    for (i, n) in input.into_iter().enumerate().skip(LEN) {
        let start = if i < LEN { 0 } else { i - LEN };
        let mut found = false;
        for m in (&lookup[start..i]).iter().flatten() {
            if n == *m {
                found = true;
                break;
            }
        }
        if !found {
            timer.print();
            println!("{}", n);
            break;
        }
    }
    Ok(())
}
