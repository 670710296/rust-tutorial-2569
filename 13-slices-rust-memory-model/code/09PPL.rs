//9.1 &collection[start..end]
fn main() {
    //&[T]
    let numbers = [10, 20, 30, 40, 50];

    let part = &numbers[1..4];

    println!("{:?}", part);

    //&str
    let text = String::from("Hello Rust");

    let word = &text[0..5];

    //Vec
    let numbers = vec![10, 20, 30, 40, 50];

    let part = &numbers[1..4];
}

//9.2 Semantics
fn main() {
    let data = [10, 20, 30, 40, 50];
    let part = &data[1..4];

    let numbers = vec![10, 20, 30, 40];
    let part = &numbers[1..3];

    let text = String::from("Hello");
    let part = &text[0..2];
}

//9.3 Type System
fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    let slice: &[i32] = &arr[1..4];

    let text: &str = "Hello Rust";

    let vec: Vec<i32> = vec![1, 2, 3, 4, 5];

    let vec_slice: &[i32] = &vec[1..4];
}

//9.4 Memory / Resource Management
fn main() {
    let numbers = vec![10, 20, 30, 40, 50];
    let part = &numbers[1..4];

    let numbers = [1, 2, 3, 4, 5];

    let data = vec![1, 2, 3, 4];
    let part = &data[1..3];
}

//9.5 Abstraction / Other PPL Concepts
fn print_data(data: &[i32]) {

    println!("{:?}", data);

}

fn main() {
    let arr = [1, 2, 3, 4];
    let vec = vec![5, 6, 7, 8];

    let data = vec![1, 2, 3];

    {
        let part = &data[0..2];
    }

    let data = vec![1, 2, 3, 4];
    let part = &data[1..3];

    let data = vec![1, 2, 3];
    let part = &data[0..2];
}

//9.6 Why Rust?
fn main() {
    let data = vec![10, 20, 30, 40, 50];
  
    let part = &data[1..4];
}










