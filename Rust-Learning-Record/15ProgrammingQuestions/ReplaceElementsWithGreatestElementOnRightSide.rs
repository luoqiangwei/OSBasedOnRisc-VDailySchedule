pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
    let mut max = -1;
    for i in (1..arr.len()).rev() {
        if arr[i] > max {
            max ^= arr[i];
            arr[i] ^= max;
            max ^= arr[i];
        }
        if max >= arr[i - 1] {
            arr[i - 1] = max;
        }
    }
    let t = arr.len()-1;
    arr[t] = -1;
    arr

    // let t = arr.len();
    // for i in 1..arr.len() {
    //     if arr[t - i] > arr[t - i - 1] {arr[t - i - 1] = arr[t - i]}
    // }
    // arr[t-1] = -1;
    // arr
}

fn main() {
    let a: Vec<i32> = vec![17, 18, 5, 4, 6, 1];
    println!("{:?}", replace_elements(a));
    // println!("{:?}", reverse_left_words(String::from("lrloseumgh"), 6));
    // println!("{:?}", reverse_left_words(String::from("abcdefg"), 2));
}