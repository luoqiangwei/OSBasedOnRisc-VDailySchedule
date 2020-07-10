pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let mut result: Vec<bool> = Vec::new();
    let mut max = -1;
    for i in 0..candies.len() {
        if candies[i] > max {
            max = candies[i];
        }
    }
    for i in 0..candies.len() {
        if candies[i] + extra_candies >= max {
            result.push(true);
        } else {
            result.push(false);
        }
    }
    result
}

fn main() {
    let a: Vec<i32> = vec![2, 3, 5, 1, 3];
    let b: Vec<i32> = vec![4, 2, 1, 1, 2];
    let c: Vec<i32> = vec![12, 1, 12];
    println!("{:?}", kids_with_candies(a, 3));
    println!("{:?}", kids_with_candies(b, 1));
    println!("{:?}", kids_with_candies(c, 10));
}