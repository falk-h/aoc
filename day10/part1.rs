use util::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timer = Timer::new();
    let mut input = input::lines::<u16>(&std::env::args().nth(1).unwrap());
    input.sort();
    let mut ones = 0;
    let mut threes = 1;
    let mut last = 0;
    for adapter in input {
        let diff = adapter - last;
        last = adapter;
        if diff == 1 {
            ones += 1;
        } else if diff == 3 {
            threes += 1;
        }
    }
    timer.print();
    println!("{}", ones * threes);
    Ok(())
}
