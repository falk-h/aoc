use itertools::Itertools;
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
                    if neighbors(matrix, x, y) >= 4 {
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
    let height = matrix.len();
    let width = matrix[0].len();
    //println!("width: {}, height: {}", width as isize, height as isize);
    let x = x as isize;
    let y = y as isize;
    (x - 1..=x + 1)
        .filter(|n| n >= &0 && n < &(width as isize))
        .cartesian_product((y - 1..=y + 1).filter(|n| n >= &0 && n < &(height as isize)))
        .filter(|(x2, y2)| {
            (*x2, *y2) != (x, y) && matrix[*y2 as usize][*x2 as usize] == Seat::Taken
        })
        .count()
}
