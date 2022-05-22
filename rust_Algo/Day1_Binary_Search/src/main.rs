mod search_insert_position;

fn main() {
    println!("Hello, world!");
}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right: i32 = (nums.len() - 1) as i32;
        while left <= right {
            let mid: i32 = (left + right) / 2;
            if nums[mid as usize] == target {
                return mid;
            }
            if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        return -1;
    }
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
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut lo: i32 = 1;
        let mut high: i32 = n;
        while lo <= high {
            let pivot: i32 = lo + (hi - lo) / 2;
            if self.isBadVersion(pivot) {
                high = pivot - 1;
            } else {
                lo = pivot + 1;
            }
        }
        return lo;
    }
    pub fn isBadVersion(version: i32) -> bool;
}
