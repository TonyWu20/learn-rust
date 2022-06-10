pub struct Solution;

impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        if s.is_empty() {
            0
        } else if s.chars().eq(s.chars().rev()) {
            1
        } else {
            2
        }
    }
}

#[test]
fn test_remove_palindrome_sub() {
    let data = vec![("ababa", 1), ("babb", 2), ("", 0)];
    for (s, i) in data {
        assert_eq!(i, Solution::remove_palindrome_sub(s.to_string()));
    }
}
