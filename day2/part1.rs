use regex::Regex;
use util::input;

const DAY: u8 = 2;

struct Requirement {
    from: usize,
    to: usize,
    character: char,
}

fn main() {
    let input = input::line(DAY);
    let timer = util::Timer::new();
    let parsed = parse(&input);
    let count = parsed.iter().filter(|(req, pass)| validate(req, pass)).count();
    timer.print();
    println!("{}", count);
}

fn validate(req: &Requirement, pass: &str) -> bool {
    let count = pass.chars().filter(|&c| c == req.character).count();
    req.from <= count && count <= req.to
}

fn parse<'a>(input: &'a String) -> Vec<(Requirement, String)> {
    let re = Regex::new(r"(?P<from>\d+)-(?P<to>\d+) (?P<character>[a-z]): (?P<pass>[a-z]+)").unwrap();
    re.captures_iter(&input)
        .map(move |cap| {
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
