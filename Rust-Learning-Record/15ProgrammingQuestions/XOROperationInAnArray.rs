pub fn xor_operation(n: i32, start: i32) -> i32 {
    if start % 2 == 0 {
        (start..(start + 2 * n)).filter(|x| x % 2 == 0).fold(0, |acc, x| acc ^ x)
    } else {
        (start..(start + 2 * n)).filter(|x| x % 2 == 1).fold(0, |acc, x| acc ^ x)
    }
}

fn main() {
    println!("{:?}", xor_operation(5, 0));
    println!("{:?}", xor_operation(4, 3));
    println!("{:?}", xor_operation(1, 7));
}