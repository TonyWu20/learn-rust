use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    /*
    Given an array of integers nums sorted in non-decreasing order, find the starting and ending position of a given target value.
    If target is not found in the array, return [-1, -1].
    You must write an algorithm with O(log n) runtime complexity.
    */
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }
        let lower: i32 = Self::lower_bound(&nums, target);
        let upper: i32 = Self::upper_bound(&nums, target);
        if lower > upper {
            return vec![-1, -1];
        }
        vec![lower, upper]
    }
    fn lower_bound(nums: &Vec<i32>, target: i32) -> i32 {
        let len: usize = nums.len();
        let (mut lo, mut hi) = (0, len);
        while lo < hi {
            let mid: usize = (lo + hi) / 2;
            if target > nums[mid] {
                lo = mid + 1;
            } else {
                hi = mid
            }
        }
        lo as i32
    }
    fn upper_bound(nums: &Vec<i32>, target: i32) -> i32 {
        let len: usize = nums.len();
        let (mut lo, mut hi) = (0, len);
        while lo < hi {
            let mid: usize = (lo + hi) / 2;
            if target < nums[mid] {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        hi as i32 - 1
    }
    /*
    # Search in Rotated Sorted Array
    There is an integer array nums sorted in ascending order (with distinct values).

    Prior to being passed to your function, nums is possibly rotated at an unknown pivot index k (1 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 and become [4,5,6,7,0,1,2].

    Given the array nums after the possible rotation and an integer target, return the index of target if it is in nums, or -1 if it is not in nums.

    You must write an algorithm with O(log n) runtime complexity.
    */
    // My ver
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        fn binary_search(nums: &Vec<i32>, target: i32, lo: usize, hi: usize) -> i32 {
            if lo > hi {
                return -1;
            }
            let mid: usize = (lo + hi) / 2;
            if nums[mid] == target {
                return mid as i32;
            }
            if nums[lo] <= nums[mid] {
                if nums[lo] <= target && target <= nums[mid] {
                    return binary_search(nums, target, lo, mid - 1);
                }
                return binary_search(nums, target, mid + 1, hi);
            }
            if nums[mid] <= target && target <= nums[hi] {
                return binary_search(nums, target, mid + 1, hi);
            }
            binary_search(nums, target, lo, mid - 1)
        }
        binary_search(&nums, target, 0, nums.len() - 1)
    }
    // Leetcode 0 ms sample
    pub fn search_2(nums: Vec<i32>, target: i32) -> i32 {
        fn bs(arr: &[i32], x: i32) -> Option<usize> {
            let n = arr.len();
            if n == 0 {
                return None;
            }
            let (mut left, mut right) = (0, n - 1);
            while left != right {
                let mid = (left + right + 1) / 2;
                if arr[mid] == x {
                    return Some(mid);
                }
                if arr[mid] > x {
                    right = mid - 1;
                } else {
                    left = mid;
                }
            }
            if arr[left] == x {
                return Some(left);
            }
            None
        }
        // find the pivot point
        let n = nums.len();
        let mut pivot = 0;
        for i in 1..n {
            if nums[i] < nums[i - 1] {
                pivot = i;
                break;
            }
        }
        // println!("Pivot: {}", pivot);
        // println!("Search 1st seg: {:?}", Self::bs(&nums[..pivot], target));
        // println!("Search 2nd seg: {:?}", Self::bs(&nums[pivot..], target));
        // perform search on both segment
        if let Some(r) = bs(&nums[..pivot], target) {
            return r as i32;
        }
        if let Some(r) = bs(&nums[pivot..], target) {
            return (pivot + r) as i32;
        }
        -1
    }
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let matrix = matrix.concat();
        let m_search = |matrix: &[i32], target: i32| -> Option<usize> {
            let (mut l, mut r) = (0, matrix.len());
            while l < r {
                let m = (l + r) / 2;
                match matrix[m as usize].cmp(&target) {
                    Ordering::Greater => r = m,
                    Ordering::Less => l = m + 1,
                    Ordering::Equal => return Some(m as usize),
                }
            }
            None
        };
        return m_search(&matrix, target).is_some();
    }
}
