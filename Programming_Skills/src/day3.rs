use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    // 976 Largest Perimeter Triangle
    // Given an integer array nums, return the largest perimeter of a triangle with a non-zero area,
    // formed from three of these lengths. If it is impossible to form any triangle of a non-zero area, return 0.
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        // nums.sort_unstable_by(|a, b| a.cmp(b).reverse());
        // let (mut a, mut b, mut c) = (0, 1, 2);
        // while c < nums.len() {
        //     if nums[a] < nums[b] + nums[c] {
        //         return nums[a] + nums[b] + nums[c];
        //     } else {
        //         a += 1;
        //         b += 1;
        //         c += 1;
        //     }
        // }
        // 0
        nums.sort_unstable();
        let mut n = nums.len();
        while n > 2 {
            if (nums[n - 3] + nums[n - 2]) > nums[n - 1] {
                return (nums[n - 3] + nums[n - 2]) + nums[n - 1];
            }
            n -= 1;
        }
        0
    }
    /* Find Nearest Point That Has The Same X or Y Coordinate
    You are given two integers, x and y, which represent your current location on a Cartesian grid: (x, y). You are also given an array points where each points[i] = [ai, bi] represents that a point exists at (ai, bi). A point is valid if it shares the same x-coordinate or the same y-coordinate as your location.
    Return the index (0-indexed) of the valid point with the smallest Manhattan distance from your current location. If there are multiple, return the valid point with the smallest index. If there are no valid points, return -1.
    The Manhattan distance between two points (x1, y1) and (x2, y2) is abs(x1 - x2) + abs(y1 - y2).
    */
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        // Imperative
        // let mut min_dist = std::i32::MAX;
        // let mut ans: i32 = -1;
        // for (idx, point) in points.iter().enumerate() {
        //     if point[0] == x || point[1] == y {
        //         let man_dist = (point[0] - x).abs() + (point[1] - y).abs();
        //         if man_dist < min_dist {
        //             min_dist = man_dist;
        //             ans = idx as i32;
        //         }
        //     }
        // }
        // ans as i32

        // Functional
        points
            .iter()
            .enumerate()
            .filter_map(|(idx, coord)| -> Option<(i32, i32)> {
                if coord[0] == x {
                    Some((idx as i32, (coord[1] - y).abs()))
                } else if coord[1] == y {
                    Some((idx as i32, (coord[0] - x).abs()))
                } else {
                    None
                }
            })
            .fold(
                (-1, i32::MAX),
                |(min_index, min_dist), (index, dist)| -> (i32, i32) {
                    if dist < min_dist {
                        (index, dist)
                    } else {
                        (min_index, min_dist)
                    }
                },
            )
            .0
    }
}

#[test]
fn test_perimeter() {
    let data = [
        (vec![2, 1, 2], 5),
        (vec![1, 2, 1], 0),
        (vec![3, 2, 3, 4], 10),
    ];
    for (nums, ans) in data {
        let perimeter = Solution::largest_perimeter(nums);
        println!("ans: {}", &perimeter);
        assert_eq!(ans, perimeter, "ans: {}", perimeter);
    }
}
#[test]
fn test_valid() {
    let data = [
        (
            3,
            4,
            vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]],
            2,
        ),
        (3, 4, vec![vec![3, 4]], 0),
    ];
    for (x, y, points, ans) in data {
        let sol = Solution::nearest_valid_point(x, y, points);
        assert_eq!(ans, sol);
    }
}
