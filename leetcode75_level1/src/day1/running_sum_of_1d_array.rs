#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = 0;
        nums.iter()
            .map(|i| -> i32 {
                sum += i;
                sum
            })
            .collect::<Vec<i32>>()
    }
}

#[test]
fn running_sum() {
    let cases: [(Vec<i32>, Vec<i32>); 3] = [
        (vec![1, 2, 3, 4], vec![1, 3, 6, 10]),
        (vec![1, 1, 1, 1, 1], vec![1, 2, 3, 4, 5]),
        (vec![3, 1, 2, 10, 1], vec![3, 4, 6, 16, 17]),
    ];
    cases.iter().for_each(|(case, ans)| {
        assert_eq!(*ans, Solution::running_sum(case.to_vec()));
    });
}
