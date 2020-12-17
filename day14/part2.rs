use util::*;
use std::collections::HashSet;

#[derive(Debug)]
enum Op {
    Mask(usize, usize),
    Write(usize, usize),
}

#[derive(Debug)]
struct Write {
    addrs: Vec<usize>,
    val: usize,
}

impl std::str::FromStr for Op {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.chars().skip(1);
        if iter.next().unwrap() == 'a' {
            let iter = iter.skip(5);
            let mut ones = 0;
            let mut floating = 0;
            for c in iter {
                ones <<= 1;
                ones |= (c == '1') as usize;
                floating <<= 1;
                floating |= (c == 'X') as usize;
            }
            Ok(Self::Mask(ones, floating))
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

fn add_floating(addrs: &mut Vec<usize>, bit_index: usize) {
    for i in 0..addrs.len() {
        addrs.push(addrs[i] ^ (1 << bit_index));
    }
}

fn expand_addr(addr: usize, ones: usize, floating: usize) -> Vec<usize> {
    let addr = ones | addr;
    let mut addrs = Vec::with_capacity(2usize.pow(8));
    addrs.push(addr);
    for index in 0..36 {
        if (floating >> index) & 1 == 1 {
            add_floating(&mut addrs, index);
        }
    }
    addrs
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timer = Timer::new();
    let input = input::lines::<Op>(&std::env::args().nth(1).unwrap());
    let mut writes: Vec<Write> = Vec::with_capacity(input.len());
    let mut ones = 0;
    let mut floating = 0;
    for op in input {
        match op {
            Op::Mask(new_ones, new_floating) => {
                ones = new_ones;
                floating = new_floating;
            }
            Op::Write(addr, val) => {
                let addrs = expand_addr(addr, ones, floating);
                writes.push(Write { addrs, val });
            }
        }
    }
    timer.print();
    let timer = Timer::new();
    let mut sum: usize = 0;
    let mut written: HashSet<usize> = HashSet::new();
    for write in writes.iter().rev() {
        for addr in write.addrs.iter() {
            if written.insert(*addr) {
                sum += write.val;
            }
        }
    }
    timer.print();
    println!("{}", sum);
    Ok(())
}
