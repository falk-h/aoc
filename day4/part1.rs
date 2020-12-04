use util::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timer = Timer::new();
    let count = input::vec::<String>(&std::env::args().nth(1).unwrap(), "\n\n")
        .iter()
        .map(|s| s.replace("\n", " "))
        .filter(|s| {
            let mut res = true;
            for field in vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] {
                res = res && s.contains(field);
            }
            res
        })
        .count();
    println!("{}", count);
    timer.print();
    Ok(())
}
