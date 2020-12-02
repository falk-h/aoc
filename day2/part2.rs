use util::*;

const DAY: u8 = 2;

parseable_struct! {Password,
    "{}-{} {}: {}",
    from: usize = "[0-9]+",
    to: usize = "[0-9]+",
    character: char = "[a-z]",
    pass: String = "[a-z]+",
}

fn main() {
    let timer = util::Timer::new();
    let input = input::lines::<Password>(DAY);
    let count = input
        .iter()
        .filter(|pass| {
            let bytes = pass.pass.as_bytes();
            let byte = pass.character as u8;
            (bytes[pass.from - 1] == byte) != (bytes[pass.to - 1] == byte)
        })
        .count();
    timer.print();
    println!("{}", count);
}
