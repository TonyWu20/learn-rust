impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut lo: i32 = 0;
        let mut hi: i32 = nums.len() as i32 - 1;
        while lo <= hi {
            let mid = (lo + hi) / 2;
            if nums[mid as usize] == target {
                return mid;
            }
            if nums[mid as usize] < target {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        return lo;
    }
}
