use util::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timer = Timer::new();
    let mut numbers = input::vec::<usize>(&std::env::args().nth(1).unwrap(), ",");
    while numbers.len() < 2020 {
        let mut distance = 0;
        if numbers
            .iter()
            .rev()
            .skip(1)
            .find(|n| {
                distance += 1;
                *n == numbers.last().unwrap()
            })
            .is_some()
        {
            numbers.push(distance);
        } else {
            numbers.push(0);
        }
    }
    timer.print();
    println!("{}", numbers.last().unwrap());
    Ok(())
}
