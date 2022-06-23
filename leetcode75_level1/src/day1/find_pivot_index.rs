#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let mut leftsum = 0;
        let mut cur: usize = 0;
        while cur < nums.len() {
            if leftsum == sum - leftsum - nums[cur] {
                return cur as i32;
            }
            leftsum += nums[cur];
            cur += 1;
        }
        -1
    }
}

#[test]
fn pivot_index() {
    let cases = [
        (vec![1, 7, 3, 6, 5, 6], 3),
        (vec![1, 2, 3], -1),
        (vec![2, 1, -1], 0),
    ];
    cases
        .iter()
        .for_each(|(case, ans)| assert_eq!(Solution::pivot_index(case.to_vec()), *ans));
}
