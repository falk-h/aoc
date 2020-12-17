use util::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timer = Timer::new();
    let input = input::lines::<String>(&std::env::args().nth(1).unwrap());
    let start: usize = input[0].parse().unwrap();
    let departures: Vec<usize> = input[1]
        .split(',')
        .filter_map(|n| n.parse::<usize>().ok())
        .collect();
    let earliest: Vec<usize> = departures
        .iter()
        .map(|d| {
            let mut n = *d;
            while n < start {
                n += d
            }
            n
        })
        .collect();
    let mut index = 0;
    let mut min = usize::MAX;
    for (i, d) in earliest.iter().enumerate() {
        if *d < min {
            index = i;
            min = *d;
        }
    }
    timer.print();
    println!("{}", (earliest[index] - start) * departures[index]);
    Ok(())
}
