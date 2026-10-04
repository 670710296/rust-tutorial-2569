// pub fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;}}
//     s.len()}

pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];}}
    &s[..]}

pub fn string_and_str() {
    let mut string = String::from("Silpakorn University");
    let literal: &'static str = "Silpakorn";

    let word = first_word(&string);
    println!("word from String : {word}");
    let word = first_word(literal);
    println!("word from literal : {word}");
    string.clear();
    println!("string after clear : {:?}", string);

    let th = String::from("รองเท้าเนรคุณ");
        println!("---------- {} ----------", th);
    println!("len : {} bytes", th.len());
    println!("char count : {} chars", th.chars().count());
    //println!("first letter : {}", &th[0..1]); // error

    println!("---------- is char boundary ----------");
    println!("1 byte : {}", th.is_char_boundary(1));
    println!("3 bytes : {}", th.is_char_boundary(3));
    println!("first letter : {}", &th[0..3]);

    println!("---------- fat pointer ownership ----------");
    let mut s1 = String::from("ญี่ปุ่น");
    let s2 = "มาแล้ว";
    s1.push_str(s2);
    println!("{}", s1);
    println!("s2 is {s2}");
}
