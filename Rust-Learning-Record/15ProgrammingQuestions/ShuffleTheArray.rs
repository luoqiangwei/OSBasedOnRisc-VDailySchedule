pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for i in 0..n {
        result.push(nums[i as usize]);
        result.push(nums[(n + i) as usize]);
    }
    result
}

fn main() {
    let a: Vec<i32> = vec![2, 5, 1, 3, 4, 7];
    let b: Vec<i32> = vec![1, 2, 3, 4, 4 ,3 ,2 ,1];
    let c: Vec<i32> = vec![1, 1, 2, 2];
    println!("{:?}", shuffle(a, 3));
    println!("{:?}", shuffle(b, 4));
    println!("{:?}", shuffle(c, 2));
}