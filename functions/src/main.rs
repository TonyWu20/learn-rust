fn main() {
    println!("Hello, world!");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn swap(nums: &mut Vec<i32>, i: usize, j: usize) {
    let temp = nums[i];
    nums[i] = nums[j];
    nums[j] = temp;
}

fn partition(nums: &mut Vec<i32>, start: usize, end: usize) -> i32 {
    let pivot = nums[end];
    let mut index = start;
    let mut i = start;
    while i < end {
        if nums[i] < pivot {
            swap(nums, i, index);
            index += 1;
        }
        i += 1;
    }
    swap(nums, index, end);
    return index as i32;
}

fn quicksort(nums: &mut Vec<i32>, start: usize, end: usize) {
    if start >= end {
        return;
    }
    let pivot = partition(nums, start, end);
    println!("=={pivot}");
    quicksort(nums, start, (pivot - 1) as usize);
    quicksort(nums, (pivot + 1) as usize, end);
}
