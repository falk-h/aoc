use util::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut nums = input::lines::<usize>(&std::env::args().nth(1).unwrap());
    let timer = util::Timer::new();
    nums.sort();
    for n in &nums {
        if let Some((hi, lo)) = find(2020 - n, &nums) {
            if let Ok(_) = nums.binary_search(&n) {
                timer.print();
                println!("{} * {} * {} = {}", n, hi, lo, n * hi * lo);
                break;
            }
        }
    }
    return Ok(());
}

fn find(target: usize, nums: &Vec<usize>) -> Option<(usize, usize)> {
    let mut hi = nums.len() - 1;
    let mut lo = 0;
    while nums[lo] + nums[hi] != target {
        if nums[lo] + nums[hi] > target {
            if hi == 0 {
                return None;
            }
            hi -= 1;
        } else {
            if lo == nums.len() - 1 {
                return None;
            }
            lo += 1;
        }
    }
    Some((nums[lo], nums[hi]))
}
