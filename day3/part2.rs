use util::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timer = Timer::new();
    let input = input::matrix::<char>(&std::env::args().nth(1).unwrap(), "");
    let mut count = 1;
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    for (right, down) in slopes {
        count *= (0..input.len())
            .step_by(down)
            .zip((0..).step_by(right))
            .filter(|&(i, j)| input[i][j % input[0].len()] == '#')
            .count();
    }
    timer.print();
    println!("{}", count);
    Ok(())
}
