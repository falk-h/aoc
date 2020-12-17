use util::*;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Seat {
    Empty,
    Taken,
    Floor,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = input::matrix::<char>(&std::env::args().nth(1).unwrap(), "");
    let timer = Timer::new();
    let matrix: Vec<Vec<Seat>> = input
        .into_iter()
        .map(|l| {
            l.into_iter()
                .map(|c| if c == 'L' { Seat::Empty } else { Seat::Floor })
                .collect()
        })
        .collect();
    let mut last = matrix.clone();
    loop {
        let next = next(&last);
        if next == last {
            break;
        }
        last = next;
    }
    timer.print();
    let count = last
        .into_iter()
        .flatten()
        .filter(|s| *s == Seat::Taken)
        .count();
    println!("{:?}", count);
    Ok(())
}

#[allow(dead_code)]
fn print(matrix: &Vec<Vec<Seat>>) {
    for line in matrix {
        for seat in line {
            print!(
                "{}",
                match seat {
                    Seat::Floor => '.',
                    Seat::Taken => '#',
                    Seat::Empty => 'L',
                }
            );
        }
        println!();
    }
    println!();
}

fn next(matrix: &Vec<Vec<Seat>>) -> Vec<Vec<Seat>> {
    let height = matrix.len();
    let width = matrix[0].len();
    let mut res = matrix.clone();
    for x in 0..width {
        for y in 0..height {
            match matrix[y][x] {
                Seat::Floor => res[y][x] = Seat::Floor,
                Seat::Taken => {
                    if neighbors(matrix, x, y) >= 5 {
                        res[y][x] = Seat::Empty
                    }
                }
                Seat::Empty => {
                    if neighbors(matrix, x, y) == 0 {
                        res[y][x] = Seat::Taken
                    }
                }
            }
        }
    }
    res
}

fn neighbors(matrix: &Vec<Vec<Seat>>, x: usize, y: usize) -> usize {
    let height = matrix.len() as isize;
    let width = matrix[0].len() as isize;
    //println!("width: {}, height: {}", width as isize, height as isize);
    let x = x as isize;
    let y = y as isize;
    let mut count = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let mut x = x + dx;
            let mut y = y + dy;
            while x >= 0 && x < width && y >= 0 && y < height {
                if matrix[y as usize][x as usize] == Seat::Taken {
                    count += 1;
                    break;
                } else if matrix[y as usize][x as usize] == Seat::Empty {
                    break;
                }
                x += dx;
                y += dy;
            }
        }
    }
    count
}
