use util::*;
mod computer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timer = Timer::new();
    let mut computer = input::into::<computer::Computer>(&std::env::args().nth(1).unwrap());
    computer.run_until_loop();
    timer.print();
    println!("{}", computer.acc());
    Ok(())
}
