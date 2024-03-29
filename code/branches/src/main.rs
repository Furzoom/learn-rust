fn main() {
    test1();
    test2();
    test3();
    test4();
    test5();
    test6();
    test7();
    test8();
}

fn test1() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn test2() {
    let n = 3;
    if n != 0 {
        println!("number was something other than zero")
    }
}

fn test3() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is {}", number);
}

fn test4() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
}

fn test5() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}

fn test6() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn test7() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("The value is {}", a[index]);
        index += 1;
    }

    for e in a {
        println!("The value is {}", e);
    }
}

fn test8() {
    for number in { 1..4 }.rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
