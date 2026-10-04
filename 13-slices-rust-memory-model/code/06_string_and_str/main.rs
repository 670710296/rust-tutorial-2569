fn main() {
    let en = String::from("Rong tao nerakhun");
    println!("---------- {} ----------", en);
    println!("len : {} bytes", en.len());
    println!("char count : {} chars", en.chars().count());
    println!("first letter : {}", &en[0..1]);

    let th = String::from("รองเท้าเนรคุณ");
        println!("---------- {} ----------", th);
    println!("len : {} bytes", th.len());
    println!("char count : {} chars", th.chars().count());
    println!("first letter : {}", &th[0..3]);
    // &th[0..1]  // <- panic!

    println!("---------- is char boundary ----------");
    println!("1 byte : {}", th.is_char_boundary(1));
    println!("3 bytes : {}", th.is_char_boundary(3));

    println!("---------- fat pointer ownership ----------");
    let mut s1 = String::from("ญี่ปุ่น");
    let s2 = "มาแล้ว";
    s1.push_str(s2);
    println!("{}", s1);
    println!("s2 is {s2}");
}
