use util::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timer = Timer::new();
    let input = input::matrix::<char>(&std::env::args().nth(1).unwrap(), "");
    let count = (0..input.len())
        .filter(|&i| input[i][(i * 3) % input[0].len()] == '#')
        .count();
    timer.print();
    println!("{}", count);
    Ok(())
}
