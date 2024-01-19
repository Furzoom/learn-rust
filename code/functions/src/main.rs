fn main() {
    another_function();
    display(10);
    print_labeled_measurement(5, 'h');
    statements();
    println!("{}", five());
    println!("{}", plus_one(10));
}

fn another_function() {
    println!("Another function!");
}

fn display(x: i32) {
    println!("The value of x is {}", x);
}

fn print_labeled_measurement(value: i32, uint_label: char) {
    println!("The measurement is: {}{}", value, uint_label);
}

fn statements() {
    let y = {
        let x = 1;
        x + 1
    };
    display(y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
