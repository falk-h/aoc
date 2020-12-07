use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use util::*;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"(?P<color>[a-z]+ [a-z]+) bags contain (?P<children>([0-9]+ [a-z]+ [a-z]+ bags?(, )?)*)."
    )
    .unwrap();
    static ref RE_CHILDREN: Regex =
        Regex::new(r"(?P<count>[0-9]+) (?P<color>[a-z]+ [a-z]+) bags?").unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timer = Timer::new();
    let input = input::lines::<String>(&std::env::args().nth(1).unwrap());
    let bags: HashMap<String, Vec<(u32, String)>> = input.iter().map(parse).collect();
    let count = count_contained(&"shiny gold".to_string(), &bags);
    timer.print();
    println!("{}", count - 1);
    Ok(())
}

fn parse(line: &String) -> (String, Vec<(u32, String)>) {
    let cap = RE.captures(line).unwrap();
    let mut contains = Vec::new();
    let color = cap["color"].to_owned();
    if let Some(children) = cap.name("children") {
        for c in children.as_str().split_terminator(", ") {
            let cap = RE_CHILDREN.captures(c).unwrap();
            contains.push((cap["count"].parse().unwrap(), cap["color"].to_owned()));
        }
    }
    (color, contains)
}

fn count_contained(color: &String, bags: &HashMap<String, Vec<(u32, String)>>) -> u32 {
    bags.get(color)
        .unwrap()
        .iter()
        .map(|(count, color)| count * count_contained(&color, bags))
        .sum::<u32>()
        + 1
}
