use num::integer::lcm;
use util::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timer = Timer::new();
    let buses: Vec<Option<usize>> = input::lines::<String>(&std::env::args().nth(1).unwrap())[1]
        .split(',')
        .map(|n| n.parse::<usize>().ok())
        .collect();

    let mut offsets: Vec<usize> = vec![];
    for (i, bus) in buses.iter().enumerate() {
        if bus.is_some() {
            offsets.push(i);
        }
    }

    let buses: Vec<usize> = buses.into_iter().filter_map(|b| b).collect();

    let mut departures = vec![0usize; buses.len()];
    for i in 1..buses.len() {
        let lcm = buses[0..i].iter().fold(1, |a, n| lcm(a, *n));
        while departures[i] != departures[0] + offsets[i] {
            if departures[i] < departures[0] + offsets[i] {
                departures[i] +=
                    ((departures[0] + offsets[i] - departures[i]) / buses[i]) * buses[i];
                while departures[i] < departures[0] + offsets[i] {
                    departures[i] += buses[i];
                }
            } else {
                for j in 0..i {
                    departures[j] += lcm;
                }
            }
        }
    }
    timer.print();
    println!("{}", departures[0]);
    Ok(())
}
