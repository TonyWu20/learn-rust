pub struct Solution;
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }
        let mut curr: Option<Box<ListNode>> = head;
        let mut next = curr.as_mut().unwrap().next.take();
        let curr_val = curr.as_ref().unwrap().val;
        let next_val = next.as_ref().unwrap().val;
        match next_val == curr_val {
            true => {
                while next.is_some() && curr.as_ref().unwrap().val == next.as_ref().unwrap().val {
                    curr = next;
                    next = curr.as_mut().unwrap().next.take();
                }
                Self::delete_duplicates(next)
            }
            false => {
                curr.as_mut().unwrap().next = Self::delete_duplicates(next);
                curr
            }
        }
    }
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        if nums.len() < 3 {
            return result;
        }
        nums.sort_unstable();
        for index in 0..nums.len() - 2 {
            if index > 0 && nums[index - 1] == nums[index] {
                continue;
            }
            let mut l = index + 1;
            let mut r = nums.len() - 1;
            if l < r {
                let target = 0 - nums[index];
                while l < r {
                    let cur_sum = nums[l] + nums[r];
                    if cur_sum == target {
                        result.push(vec![nums[index], nums[l], nums[r]]);
                        l += 1;
                        while nums[l - 1] == nums[l] && l < r {
                            l += 1;
                        }
                    } else if cur_sum < target {
                        l += 1;
                    } else {
                        r -= 1;
                    }
                }
            }
        }
        result
    }
}
