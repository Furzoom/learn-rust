fn main() {
    // - scalar
    //   - integer: i8, u8, i16, u16, i32, u32, i64, u64,
    //     i128, u128, isize, usize
    //   - floating-point number: f64, f32
    //   - boolean: true, false
    //   - char:
    // - compound type
    //   - tuple
    //   - array

    let a1: i8 = 10;
    let a2: u8 = b'A';
    let a3: i16 = 0x7FFF;
    let a4: u16 = 0xFFFF;
    let a5: i32 = 98_222;
    let a6: u32 = 0o77;
    let a7: i64 = 0b1111_0000_1111_0101;
    let a8: u128 = 0xFFFF_FFFF_FFFF_FFFF_FFFF_1111_FFFF_FFFF;
    let a9: isize = 0b1010_0101;
    println!(
        "{} {} {} {} {} {} {} {} {}",
        a1, a2, a3, a4, a5, a6, a7, a8, a9
    );
    let b1 = 2.0; // f64
    let b2: f32 = 3.0; // f32
    println!("{} {}", b1, b2);

    //
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;
    let remainder = 43 % 5;
    println!(
        "{} {} {} {} {} {}",
        sum, difference, product, quotient, floored, remainder
    );

    // bool
    let c1 = true;
    let c2: bool = false;
    println!("{} {}", c1, c2);

    // char
    let d1 = 'z';
    let d2 = 'Z';
    let d3 = '😻';
    println!("{} {} {}", d1, d2, d3);

    // tuple
    let t1: (i32, f64, u8) = (500, 6.4, 1);
    let (e1, e2, e3) = t1;
    println!("{} {} {}", e1, e2, e3);
    println!("{} {} {}", t1.0, t1.1, t1.2);

    // array
    let f1 = [1, 2, 3, 4];
    for i in f1 {
        print!("{} ", i);
    }
    println!();
    let f2: [u32; 3] = [10, 8, 9];
    let f3 = [3; 4];
    for i in f2 {
        print!("{} ", i);
    }
    println!();
    for i in f3 {
        print!("{} ", i);
    }
    println!();
}
