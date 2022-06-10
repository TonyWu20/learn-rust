pub struct Solution;

impl Solution {
    /* 153. Find Minimum in Rotated Sorted Array
    Suppose an array of length n sorted in ascending order is rotated between 1 and n times. For example, the array nums = [0,1,2,4,5,6,7] might become:

    [4,5,6,7,0,1,2] if it was rotated 4 times.
    [0,1,2,4,5,6,7] if it was rotated 7 times.
    Notice that rotating an array [a[0], a[1], a[2], ..., a[n-1]] 1 time results in the array [a[n-1], a[0], a[1], a[2], ..., a[n-2]].

    Given the sorted rotated array nums of unique elements, return the minimum element of this array.

    You must write an algorithm that runs in O(log n) time.
    */
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        // if n == 1 { return nums[0]; }
        let (mut l, mut r) = (0, n - 1);
        while l != r {
            let mid = (l + r) / 2;
            if nums[mid] > nums[mid + 1] {
                return nums[mid + 1];
            }
            if nums[mid] > nums[r] {
                l = mid;
            } else {
                r = mid;
            }
        }
        nums[l]
    }
    /* 162. Find Peak Element
    A peak element is an element that is strictly greater than its neighbors.

    Given an integer array nums, find a peak element, and return its index. If the array contains multiple peaks, return the index to any of the peaks.

    You may imagine that nums[-1] = nums[n] = -∞.

    You must write an algorithm that runs in O(log n) time.
    */
    pub fn find_peak_elements(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut l, mut r) = (0, n - 1);
        while l < r {
            let mid = (l + r) / 2;
            if nums[mid] > nums[mid + 1] {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l as i32
    }
}
