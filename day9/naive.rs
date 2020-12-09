use util::*;

const LEN: usize = 25;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timer = Timer::new();
    let input = input::lines::<usize>(&std::env::args().nth(1).unwrap());

    let mut target = 0;
    for (i, n) in input.iter().enumerate().skip(LEN) {
        let start = if i < LEN { 0 } else { i - LEN };
        let mut nums: Vec<&usize> = input[start..i].iter().collect();
        nums.sort();
        let mut found = false;
        let mut first = 0;
        let mut last = nums.len() - 1;
        while last != first {
            let sum = nums[first] + nums[last];
            if sum > *n {
                last -= 1;
            } else if sum < *n {
                first += 1;
            } else {
                found = true;
                break;
            }
        }
        if !found {
            timer.print();
            println!("{}", n);
            target = *n;
            break;
        }
    }

    let timer = Timer::new();

    let mut start = 0;
    'outer: while start < input.len() {
        let mut end = start + 1;
        while end < input.len() {
            let range = &input[start..=end];
            let sum: usize = range.iter().sum();
            if sum == target {
                let min = range.iter().min().unwrap();
                let max = range.iter().max().unwrap();
                timer.print();
                println!("{}", min + max);
                break 'outer;
            }
            end += 1;
        }
        start += 1;
    }

    Ok(())
}
