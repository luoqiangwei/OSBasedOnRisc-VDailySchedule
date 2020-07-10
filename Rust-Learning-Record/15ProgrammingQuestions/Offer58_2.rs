pub fn reverse_left_words(s: String, n: i32) -> String {
    (s[(n as usize)..(s.len() as usize)]).to_string() + &s[(0 as usize)..(n as usize)]
}

fn main() {
    println!("{:?}", reverse_left_words(String::from("abcdefg"), 2));
    println!("{:?}", reverse_left_words(String::from("lrloseumgh"), 6));
    // println!("{:?}", reverse_left_words(String::from("abcdefg"), 2));
}