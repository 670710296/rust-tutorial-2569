fn main() {
    let text = "Hello Rust";

    let part: &str = &text[0..5];

    println!("{}", part);
}
