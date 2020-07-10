pub fn subtract_product_and_sum(n: i32) -> i32 {
    let x = n.to_string();
    let tmp = x.chars().map(|x| x.to_digit(10).unwrap() as i32);
    tmp.clone().fold(1, |acc, x| acc * x) - tmp.fold(0, |acc, x| acc + x)
}

fn main() {
    println!("{:?}", subtract_product_and_sum(234));
    println!("{:?}", subtract_product_and_sum(4421));
    // println!("{:?}", reverse_left_words(String::from("abcdefg"), 2));
}