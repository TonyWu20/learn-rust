pub struct Solution;

impl Solution {
    /*
    Number of 1 Bits:
    Write a function that takes an unsigned integer and returns the number of '1' bits it has (also known as the Hamming weight).
    */
    pub fn hammingWeight(mut n: u32) -> i32 {
        let mut count: i32 = 0;
        while n > 0 {
            n = n & n - 1;
            count += 1;
        }
        count
    }
    /*
    Subtract the Product and Sum of Digits of an Integer
    Given an integer number n, return the difference between the product of its digits and the sum of its digits.
    */
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        // let mut product: i32 = 1;
        // let mut sum: i32 = 0;
        // while n > 0 {
        //     let digit = n % 10;
        //     sum += digit;
        //     product *= digit;
        //     n /= 10;
        // }
        // product - sum
        let string = n.to_string();
        let digits = string.chars().map(|c| c.to_digit(10).unwrap() as i32);
        let product: i32 = digits.clone().product();
        let sum: i32 = digits.sum();
        product - sum
    }
}
