use util::input;

const DAY: u8 = 1;

fn main() {
    let mut nums = input::lines::<usize>(DAY);
    nums.sort();
    for n in &nums {
        if let Some((hi, lo)) = find(2020 - n, &nums) {
            if let Ok(_) = nums.binary_search(&n) {
                println!("{} * {} * {} = {}", n, hi, lo, n * hi * lo);
            }
        }
    }
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
