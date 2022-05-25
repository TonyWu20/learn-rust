fn main() {
    println!("Hello, world!");
}

pub struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        use std::collections::HashMap;

        let words = s.split_whitespace().collect::<Vec<_>>();

        if pattern.len() != words.len() {
            return false;
        }
        let mut f_map = HashMap::<char, String>::new();
        let mut r_map = HashMap::<String, char>::new();

        for (c, word) in pattern.chars().zip(words.iter()) {
            if let Some(x) = f_map.get(&c) {
                if x != word {
                    return false;
                }
            }
            if let Some(x) = r_map.get(&word.to_string()) {
                if *x != c {
                    return false;
                }
            }
            f_map.insert(c, word.to_string());
            r_map.insert(word.to_string(), c);
        }
        println!("{:#?}\n{:#?}", f_map, r_map);
        true
    }
    pub fn partition_labels(s: String) -> Vec<i32> {
        use std::collections::HashMap;

        let mut last_id_map = HashMap::<char, usize>::new();
        let mut anchor: usize = 0;
        let mut start: usize = 0;
        let mut result: Vec<i32> = Vec::new();
        for (id, char) in s.char_indices() {
            last_id_map.insert(char, id);
        }
        for (id, char) in s.char_indices() {
            if let Some(x) = last_id_map.get(&char) {
                if *x > anchor {
                    anchor = *x;
                }
            }
            if id == anchor {
                result.push(((id - start) as i32) + 1);
                start = id + 1;
            }
        }
        result
    }
}
