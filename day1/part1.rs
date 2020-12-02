use util::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut nums = input::lines::<usize>(&std::env::args().nth(1).unwrap());
    nums.sort();
    let mut hi = nums.len() - 1;
    let mut lo = 0;
    while nums[lo] + nums[hi] != 2020 {
        if nums[lo] + nums[hi] > 2020 {
            hi -= 1;
        } else {
            lo += 1;
        }
    }
    println!("{}", nums[hi] * nums[lo]);
    Ok(())
}
