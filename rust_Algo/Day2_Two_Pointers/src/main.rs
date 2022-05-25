fn main() {
    println!("Hello, world!");
}
pub struct Solution;
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut lo: usize = 0;
        let mut hi: usize = (nums.len()) as usize - 1;
        let mut results: Vec<i32> = vec![0; nums.len()];
        let mut ptr = hi;
        while lo < hi {
            if nums[lo].abs() > nums[hi].abs() {
                results[ptr] = nums[lo] * nums[lo];
                lo += 1;
                ptr -= 1;
            } else {
                results[ptr] = nums[hi] * nums[hi];
                hi -= 1;
                ptr -= 1;
            }
        }
        results[ptr] = nums[lo] * nums[lo];
        return results;
    }
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        fn reverse(nums: &mut Vec<i32>, lo: usize, hi: usize) {
            let mut lo = lo;
            let mut hi = hi;
            while lo < hi {
                let tmp = nums[lo];
                nums[lo] = nums[hi];
                nums[hi] = tmp;
                lo += 1;
                hi -= 1;
            }
        }
        if nums.len() > 1 {
            let k = (k as usize) % nums.len();
            let n = nums.len() - (1 as usize);
            reverse(nums, 0, n - k);
            reverse(nums, n - k + 1, n);
            reverse(nums, 0, n);
        }
    }
}
