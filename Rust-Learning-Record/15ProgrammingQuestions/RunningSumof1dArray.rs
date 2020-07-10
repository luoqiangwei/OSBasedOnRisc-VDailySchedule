// running-sum-of-1d-array
pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
    for i in 1..nums.len() {
        nums[i] += nums[i - 1];
    }
    nums
}

fn main() {
    let a: Vec<i32> = vec![1, 2, 3, 4];
    let b: Vec<i32> = vec![1, 1, 1, 1, 1];
    let c: Vec<i32> = vec![3, 1, 2, 10, 1];
    println!("{:?}", running_sum(a));
    println!("{:?}", running_sum(b));
    println!("{:?}", running_sum(c));
}