use util::input;

const DAY: u8 = 1;

fn main() {
    let mut nums = input::lines::<usize>(DAY);
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
}
