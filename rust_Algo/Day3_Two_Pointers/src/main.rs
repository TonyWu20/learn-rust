fn main() {
    println!("Hello, world!");
}

pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut non_zero_pos: usize = 0;
        let mut i: usize = 0;
        while i < nums.len() {
            if nums[i] != 0 {
                nums[non_zero_pos] = nums[i];
                if non_zero_pos != i {
                    nums[i] = 0;
                }
                non_zero_pos += 1;
            }
            i += 1;
        }
    }
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut lo: usize = 0;
        let mut hi: usize = numbers.len() - 1;
        while lo < hi {
            let curr: i32 = numbers[lo] + numbers[hi];
            if curr == target {
                break;
            }
            if curr > target {
                hi -= 1;
            } else {
                lo += 1;
            }
        }
        let result: Vec<i32> = vec![(lo as i32) + 1, (hi as i32) + 1];
        return result;
    }
}
