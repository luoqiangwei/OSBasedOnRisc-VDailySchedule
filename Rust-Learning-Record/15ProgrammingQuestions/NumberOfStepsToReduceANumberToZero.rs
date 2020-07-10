pub fn number_of_steps (mut num: i32) -> i32 {
    let mut count: i32 = 0;
    while num > 0 {
        num = if num % 2 == 0 {num / 2} else {num - 1};
        count += 1;
    }
    count
}

fn main() {
    println!("{:?}", number_of_steps(14));
    println!("{:?}", number_of_steps(8));
    println!("{:?}", number_of_steps(123));
}