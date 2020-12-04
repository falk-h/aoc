use anyhow::anyhow;
use lazy_static::lazy_static;
use regex::Regex;
use util::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timer = Timer::new();
    let count = input::vec::<String>(&std::env::args().nth(1).unwrap(), "\n\n")
        .iter()
        .filter(|s| check(s).unwrap_or(false))
        .count();
    timer.print();
    println!("{}", count);
    Ok(())
}

lazy_static! {
    static ref BYR: Regex = Regex::new(r"byr:([0-9]+)").unwrap();
    static ref IYR: Regex = Regex::new(r"iyr:([0-9]+)").unwrap();
    static ref EYR: Regex = Regex::new(r"eyr:([0-9]+)").unwrap();
    static ref HGT: Regex = Regex::new(r"hgt:([0-9]+)(cm|in)").unwrap();
    static ref HCL: Regex = Regex::new(r"hcl:#[0-9a-f]{6}([^0-9a-f]|$)").unwrap();
    static ref ECL: Regex = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
    static ref PID: Regex = Regex::new(r"pid:[0-9]{9}([^0-9]|$)").unwrap();
}

fn check(line: &String) -> Result<bool, anyhow::Error> {
    let byr = BYR.captures(line).ok_or(anyhow!("no byr"))?[1].parse::<u16>()?;
    let iyr = IYR.captures(line).ok_or(anyhow!("no iyr"))?[1].parse::<u16>()?;
    let eyr = EYR.captures(line).ok_or(anyhow!("no eyr"))?[1].parse::<u16>()?;
    HCL.captures(line).ok_or(anyhow!("no hcl"))?;
    ECL.captures(line).ok_or(anyhow!("no ecl"))?;
    PID.captures(line).ok_or(anyhow!("no pid"))?;

    let hgt_cap = HGT.captures(line).ok_or(anyhow!("no hgt"))?;
    let hgt = hgt_cap[1].parse::<u8>()?;
    let hgt_range = if &hgt_cap[2] == "cm" {
        150..=193
    } else {
        59..=76
    };

    Ok((1920..=2002).contains(&byr)
        && (2010..=2020).contains(&iyr)
        && (2020..=2030).contains(&eyr)
        && hgt_range.contains(&hgt))
}
