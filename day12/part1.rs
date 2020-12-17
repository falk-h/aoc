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

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
enum Dir {
    East = 0,
    South = 1,
    West = 2,
    North = 3,
}

fn direction(dir: Dir) -> (i32, i32) {
    match dir {
        Dir::East => (1, 0),
        Dir::South => (0, -1),
        Dir::North => (0, 1),
        Dir::West => (-1, 0),
    }
}

fn rotate(dir: Dir, rotation: i32) -> Dir {
    const LOOKUP: [Dir; 4] = [Dir::East, Dir::South, Dir::West, Dir::North];
    LOOKUP[((rotation / 90) as usize + dir as usize) % LOOKUP.len()]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timer = Timer::new();
    let mut x = 0;
    let mut y = 0;
    let mut dir = Dir::East;
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
            Instruction::North(d) => y += d,
            Instruction::South(d) => y -= d,
            Instruction::West(d) => x -= d,
            Instruction::East(d) => x += d,
            Instruction::Left(a) => dir = rotate(dir, -a),
            Instruction::Right(a) => dir = rotate(dir, a),
            Instruction::Forward(d) => {
                let (dx, dy) = direction(dir);
                x += dx * d;
                y += dy * d;
            }
        }
    });
    timer.print();
    println!("{}", x.abs() + y.abs());
    Ok(())
}