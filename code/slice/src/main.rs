fn main() {
    test_first_word();
    test_slice();
    test_str_literal();
    test_first_word_v3();
    test_other_slice();
}

fn test_first_word() {
    let s = String::from("hello world");
    let n = first_word(&s);
    println!("'{}' first word at {}", s, n);

    println!("'{}' first word is {}", s, first_word_v2(&s));
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn test_slice() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);
}

fn first_word_v2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn test_str_literal() {
    let mut s = "hello";
}

fn first_word_v3(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn test_first_word_v3() {
    let s1 = String::from("hello world");
    let word = first_word_v3(&s1[0..6]);
    println!("'{}' first word is {}", s1, word);
    let word = first_word_v3(&s1[..]);
    println!("'{}' first word is {}", s1, word);
    let word = first_word_v3(&s1);
    println!("'{}' first word is {}", s1, word);

    let s1 = "hello world";
    let word = first_word_v3(&s1[0..6]);
    println!("'{}' first word is {}", s1, word);
    let word = first_word_v3(&s1[..]);
    println!("'{}' first word is {}", s1, word);
    let word = first_word_v3(&s1);
    println!("'{}' first word is {}", s1, word);
}

fn test_other_slice() {
    let a = [1, 2, 3, 4, 5];
    let a1 = &a[1..3];
    for i in a1 {
        print!("{} ", i)
    }
    println!();
}
