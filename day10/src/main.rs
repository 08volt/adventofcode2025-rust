use std::fs;

fn read_txt_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("Could not open file: {}", path))
}

fn main() {
    let content: String = read_txt_file("input.txt");
    let x: i64 = 0;
    // count chars in content
    let count = content.chars().count() as i64;

    // multiply count by 10
    println!("{} and {}", x, count);
    println!("{}", content);
}
