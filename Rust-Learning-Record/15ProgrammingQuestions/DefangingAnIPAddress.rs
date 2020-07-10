pub fn defang_i_paddr(address: String) -> String {
    address.replace(".", "[.]")  
}

fn main() {
    println!("{:?}", defang_i_paddr(String::from("1.1.1.1")));
    println!("{:?}", defang_i_paddr(String::from("255.100.50.0")));
    // println!("{:?}", reverse_left_words(String::from("abcdefg"), 2));
}