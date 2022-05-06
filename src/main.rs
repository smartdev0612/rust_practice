fn main() {
    // Initializing multiple variables
    let (first_number, second_number) = (250, 480.32);
    println!("The first number is {} and the second number is {}", first_number, second_number);

    // Readability of large number
    let large_number = 1_000_000;
    println!("The value of the large number is {}", large_number);
    
    // Integer overflow
    // let overflow_number:u8 = 256;

    // Decimal numbers in other formats
    let x = 255;
    println!("The value of variable x in hexadecimal is {:o} and in octal is {:X} and in binary {:b}", x, x, x);

    // Snake case convention
    let number = 45;

    // Operations on number in different formats
    let n1 = 14;
    let n2 = 15.6;
    let n3 = n1 as f64 + n2;
    println!("{}", n3);

    // shadowing
    let s = 5;
    let s = 5 * 5;
    println!("the value of the variable s = {}", s);

    // string
    let some_string = "Fixed Length string";
    println!("The text inside the string is \"{}\"", some_string);

    let my_name = "Nouman azam".to_string();

    let p1 = (2.5, 3.5);
    let p2 = (1.5, 5.5);

    println!("Diff X: {}", (p1.0 - p2.0 as f64).abs());
    println!("Diff Y: {}", (p1.1 - p2.1 as f64).abs());

    let p3 = [2.5, 3.5];
    let p4 = [1.5, 5.7];

    println!("Diff X: {}", (p3[0] - p4[0] as f64).abs());
    println!("Diff Y: {}", (p3[1] - p4[1] as f64).abs());

    let p5 = (4.0, 3.0);
    let p6 = (5.0, 4.5);
    let distance = (((p5.0 - p6.0) as f64).powf(2.0) + ((p5.1 - p6.1) as f64).powf(2.0)).sqrt();

    println!("Distance: {}", distance);
}


