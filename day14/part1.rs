use util::*;

#[derive(Debug)]
enum Op {
    Mask(usize, usize),
    Write(usize, usize),
}

impl std::str::FromStr for Op {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.chars().skip(1);
        if iter.next().unwrap() == 'a' {
            let iter = iter.skip(5);
            let mut ones = 0;
            let mut zeroes = 0;
            for c in iter {
                ones <<= 1;
                ones |= (c == '1') as usize;
                zeroes <<= 1;
                zeroes |= (c == '0') as usize;
            }
            Ok(Self::Mask(ones, zeroes))
        } else {
            let index_end = s.find(']').unwrap();
            let val_start = index_end + 4;
            Ok(Self::Write(
                s[4..index_end].parse().unwrap(),
                s[val_start..].parse().unwrap(),
            ))
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timer = Timer::new();
    let input = input::lines::<Op>(&std::env::args().nth(1).unwrap());
    let mut nums = [0; 100_000];
    let (mut ones, mut zeroes) = (0, 0);
    for op in input {
        match op {
            Op::Mask(new_ones, new_zeroes) => {
                ones = new_ones;
                zeroes = new_zeroes;
            }
            Op::Write(index, value) => {
                nums[index] = ones | (value & !ones & !zeroes);
            }
        }
    }
    timer.print();
    println!("{}", nums.iter().sum::<usize>());
    Ok(())
}
