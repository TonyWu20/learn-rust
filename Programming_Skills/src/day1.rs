pub struct Solution;

impl Solution {
    /*
    Given two non-negative integers low and high. Return the count of odd numbers between low and high (inclusive).
    */
    pub fn count_odds(low: i32, high: i32) -> i32 {
        match (low % 2, high % 2) {
            (1, 1) => (high - low) / 2 + 1,
            (0, 0) => (high - low) / 2,
            _ => (high - low) / 2 + 1,
        }
    }
    pub fn count_odds_2(low: i32, high: i32) -> i32 {
        let a: i32 = (high - low + 1) % 2;
        let b = if (high + low) % 2 == 0 { low % 2 } else { 0 };
        a + b
    }
    /*
    You are given an array of unique integers salary where salary[i] is the salary of
    the ith employee. Return the average salary of employees excluding the minimum and
    maximum salary. Answers within 10-5 of the actual answer will be accepted.
    */
    pub fn average(mut salary: Vec<i32>) -> f64 {
        salary.sort_unstable();
        (salary[2..salary.len() - 1].iter().sum::<i32>() as f64) / (salary.len() - 2) as f64
    }
}
