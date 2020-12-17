use util::*;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    North(i32),
    South(i32),
    West(i32),
    East(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

fn rotate(wx: &mut i32, wy: &mut i32, rotation: i32) {
    match rotation.rem_euclid(360) {
        90 => {
            let tmp = *wy;
            *wy = -*wx;
            *wx = tmp;
        }
        180 => {
            *wy = -*wy;
            *wx = -*wx;
        }
        270 => {
            let tmp = -*wy;
            *wy = *wx;
            *wx = tmp;
        }
        _ => panic!("Invalid rotation ({})", rotation),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timer = Timer::new();
    let mut x = 0;
    let mut y = 0;
    let mut wx = 10;
    let mut wy = 1;
    let input = input::lines::<String>(&std::env::args().nth(1).unwrap());
    input.iter().map(|l| match &l[0..1] {
        "N" => Instruction::North(l[1..].parse().unwrap()),
        "E" => Instruction::East(l[1..].parse().unwrap()),
        "S" => Instruction::South(l[1..].parse().unwrap()),
        "W" => Instruction::West(l[1..].parse().unwrap()),
        "F" => Instruction::Forward(l[1..].parse().unwrap()),
        "L" => Instruction::Left(l[1..].parse().unwrap()),
        "R" => Instruction::Right(l[1..].parse().unwrap()),
        _ => panic!("Invalid input. Got \"{}\"", l),
    }).for_each(|i|{
        match i {
            Instruction::North(d) => wy += d,
            Instruction::South(d) => wy -= d,
            Instruction::West(d) => wx -= d,
            Instruction::East(d) => wx += d,
            Instruction::Left(a) => rotate(&mut wx, &mut wy, -a),
            Instruction::Right(a) => rotate(&mut wx, &mut wy, a),
            Instruction::Forward(d) => {
                x += wx * d;
                y += wy * d;
            }
        }
    });
    timer.print();
    println!("{}", x.abs() + y.abs());
    Ok(())
}