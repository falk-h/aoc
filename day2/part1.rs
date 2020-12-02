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
    let input = input::lines::<Password>(DAY);
    let timer = Timer::new();
    let count = input
        .iter()
        .filter(|pass| {
            let count = pass.pass.chars().filter(|&c| c == pass.character).count();
            pass.from <= count && count <= pass.to
        })
        .count();
    timer.print();
    println!("{}", count);
}
