use std::{cmp::max, collections::HashMap};

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut sub_str_start: usize = 0;
        let mut longest = 0;
        let mut map: HashMap<char, usize> = HashMap::new();
        for (idx, c) in s.char_indices() {
            map.entry(c)
                .and_modify(|old_idx| {
                    if *old_idx >= sub_str_start {
                        longest = max(longest, idx - sub_str_start);
                        sub_str_start = *old_idx + 1;
                    }
                    *old_idx = idx;
                })
                .or_insert(idx);
        }
        max(longest, s.len() - sub_str_start) as i32
    }
}

#[test]
fn some_test() {
    let data = vec![
        ("abcabcbb", 3),
        ("bbbbb", 1),
        ("pwwkew", 3),
        ("a", 1),
        ("AAACDEFGH", 7),
    ];
    for (s, l) in data {
        let length = Solution::length_of_longest_substring(s.to_string());
        assert_eq!(l, length, "for {}", s);
    }
}
