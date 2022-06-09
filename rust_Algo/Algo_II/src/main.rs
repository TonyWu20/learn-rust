mod day1;

fn main() {
    println!("Hello, world!");
}
#[test]
fn test_bs() {
    use day1::Solution;

    let nums: Vec<i32> = vec![4, 5, 6, 7, 0, 1, 2];
    println!("Res: {}", Solution::search(nums, 0));
}
