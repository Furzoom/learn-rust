fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);
    one_string();
    ownership();
    move_ownership();
    multi_values();
}

fn one_string() {
    let s1 = String::from("hello");
    // let s2 = s1;  // borrow of moved value: `s1`
    let s2 = s1.clone();
    println!("{} {} world!", s1, s2);

    let x = 5;
    let y = x.clone();
    println!("{} {}", x, y);
}

fn ownership() {
    let s = String::from("hello");
    take_ownership(s);
    // println!("{}", s); // error

    let x = 3;
    makes_copy(x);
    println!("{}", x); // we can use x
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(i: i32) {
    println!("{}", i);
}

fn move_ownership() {
    let s1 = gives_ownership();
    println!("{}", s1);

    let s2 = String::from("world");
    let s3 = takes_and_gives_back(s2);
    println!("{}", s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn multi_values() {
    let s = String::from("hello");
    let (s2, len) = calculate_length(s);
    println!("{} {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
