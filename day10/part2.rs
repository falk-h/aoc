use ndarray::Array2;
use util::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timer = Timer::new();
    let mut input = input::lines::<u8>(&std::env::args().nth(1).unwrap());
    input.sort();
    let mut adapters = vec![0; 1];
    adapters.append(&mut input);
    adapters.push(adapters[adapters.len() - 1] + 3);
    let len = adapters.len();
    let mut adjacency = Array2::<usize>::zeros((len, len));
    for i in 0..len - 1 {
        for j in i + 1..len {
            if adapters[j] - adapters[i] <= 3 {
                adjacency[[i, j]] += 1;
            } else {
                break;
            }
        }
    }
    let mut sum: usize = 0;
    let mut res = adjacency.clone();
    for _ in 0..len - 2 {
        res = res.dot(&adjacency);
        sum += res[[0, len - 1]] as usize;
    }
    println!("{}", sum);
    timer.print();
    Ok(())
}
