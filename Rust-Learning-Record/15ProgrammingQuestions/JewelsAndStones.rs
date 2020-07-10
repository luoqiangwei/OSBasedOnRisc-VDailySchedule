pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
    s.chars().into_iter().filter(|&x| j.contains(x)).count() as i32
}

fn main() {
    println!("{:?}", num_jewels_in_stones(String::from("aA"), String::from("aAAbbbb")));
    println!("{:?}", num_jewels_in_stones(String::from("z"), String::from("ZZ")));
    // println!("{:?}", reverse_left_words(String::from("abcdefg"), 2));
}