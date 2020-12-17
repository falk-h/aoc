use std::collections::HashMap;
use util::*;

const ITERATIONS: usize = 30_000_000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timer = Timer::new();
    let numbers = input::vec::<usize>(&std::env::args().nth(1).unwrap(), ",");
    let mut map: HashMap<usize, usize> = HashMap::new();
    for (i, n) in numbers.iter().enumerate().take(numbers.len() - 1) {
        map.insert(*n, i);
    }
    let mut count = map.len();
    let mut last = *numbers.last().unwrap();
    while count <= ITERATIONS {
        let to_insert = if let Some(&last_pos) = map.get(&last) {
            count - last_pos
        } else {
            0
        };
        map.insert(last, count);
        last = to_insert;
        count += 1;
    }
    println!(
        "{}",
        map.iter().find(|(_, &v)| v == ITERATIONS - 1).unwrap().0
    );
    timer.print();
    Ok(())
}
