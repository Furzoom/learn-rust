fn main() {
    test_none_move();
    test_change();
    test_multi_reference();
    test_use_multi_reference();
    test_use_multi_reference2();
}

fn test_none_move() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("{}: {}", s, len);
}

fn calculate_length(s: &String) -> usize {
    println!("{} {}", s, &s);
    // cannot borrow `*s` as mutable, as it is behind a `&` reference
    // s.push_str(", world!");
    s.len()
}

fn test_change() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    let r1 = &mut s;
    // cannot borrow `s` as mutable more than once at a time
    // let r2 = &mut s;
    // println!("{} {}", r1, r2);
    println!("{}", r1);
}

fn change(s: &mut String) {
    s.push_str(", world!");
}

fn test_multi_reference() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut 2;
}

fn test_use_multi_reference() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;
    // cannot borrow `s` as mutable because it is also borrowed as immutable
    // println!("{} {} {}", r1, r2, r3);
}

fn test_use_multi_reference2() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);
    let r3 = &mut s;
    println!("{}", r3);
}

fn test_dangling_reference() {
    let s = dangle();
}

// missing lifetime specifier
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
