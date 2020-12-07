use regex::Regex;
use util::*;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static!{
    static ref RE: Regex = Regex::new(r"(?P<color>[a-z]+ [a-z]+) bags contain (?P<children>([0-9]+ [a-z]+ [a-z]+ bags?(, )?)*).").unwrap();
    static ref RE_CHILDREN: Regex = Regex::new(r"(?P<count>[0-9]+) (?P<color>[a-z]+ [a-z]+) bags?").unwrap();
}

#[derive(Debug)]
struct Bag {
    contains: Vec<(u32, String)>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timer = Timer::new();
    let input = input::lines::<String>(&std::env::args().nth(1).unwrap());
    let bags: HashMap<String, Vec<(u32, String)>> = input.iter().map(parse).collect();
    let mut count = 0;
    for (color, _) in &bags {
        if color != "shiny gold" && contains_golden(&color, &bags) {
            count += 1;
        }
    }
    timer.print();
    println!("{}", count);
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

fn contains_golden(color: &String, bags: &HashMap<String, Vec<(u32, String)>>) -> bool {
    if color == "shiny gold" {
        return true;
    }
    let children = bags.get(color).unwrap();
    for (_, color) in children.iter() {
        if contains_golden(color, bags) {
            return true;
        }
    }
    false
}
