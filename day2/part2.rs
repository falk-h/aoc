use regex::Regex;
use util::input;

const DAY: u8 = 2;

struct Requirement {
    from: usize,
    to: usize,
    character: char,
}

fn main() {
    let input = input::lines(DAY);
    let timer = util::Timer::new();
    let count = parse(input).iter().filter(|(req, pass)| validate(req, pass)).count();
    timer.print();
    println!("{}", count);
}

fn validate(req: &Requirement, pass: &str) -> bool {
    let bytes = pass.as_bytes();
    let byte = req.character as u8;
    (bytes[req.from - 1] == byte) != (bytes[req.to - 1] == byte)
}

fn parse<'a>(input: Vec<String>) -> Vec<(Requirement, String)> {
    let re: Regex = Regex::new(r"^(?P<from>[0-9]+)-(?P<to>[0-9]+) (?P<character>[a-z]): (?P<pass>[a-z]+)$").unwrap();
    input
        .iter()
        .map(|s| {
            let cap = re.captures(s).unwrap();
            (
                Requirement {
                    from: cap["from"].parse().unwrap(),
                    to: cap["to"].parse().unwrap(),
                    character: cap["character"].parse().unwrap(),
                },
                cap["pass"].to_owned(),
            )
        })
        .collect()
}
