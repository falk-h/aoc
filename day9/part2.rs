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

    let mut target = 0;
    for (i, n) in input.iter().enumerate().skip(LEN) {
        let start = if i < LEN { 0 } else { i - LEN };
        let mut found = false;
        for m in (&lookup[start..i]).iter().flatten() {
            if n == m {
                found = true;
                break;
            }
        }
        if !found {
            target = *n;
            break;
        }
    }

    let mut start = 0;
    'outer: while start < input.len() {
        let mut sum = 0;
        let mut end = start;
        for n in &input[start..] {
            sum += n;
            if *n > target {
                break;
            } else if sum == target && start != end {
                let max = input[start..=end].iter().max().unwrap();
                let min = input[start..=end].iter().min().unwrap();
                timer.print();
                println!("{} + {} = {}", min, max, min + max);
                break 'outer;
            }
            end += 1;
        }
        start += 1;
    }

    Ok(())
}
