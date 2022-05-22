impl Solution {
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
