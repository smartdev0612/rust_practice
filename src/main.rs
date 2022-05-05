fn main() {
    // Variables in Rust
    let mut x:i64 = 15;
    println!("The value of the variable x = {}", x);
    x = 60;

    // Varialbe Type
    println!("The maximum number in i8 is equal to {}", std::i8::MAX);
    println!("The maximum number in u8 is equal to {}", std::u8::MAX);

    // Floats
    let z = 3.65;
    println!("The maximum number in f32 is {}", std::f32::MAX);

    // Boolean
    let status = false;
    println!("The values of the some of our variables are {:?}", (x, z, status));

    let not_equals = 18 != 18;
    println!("The value of condition 18 != 18 is {}", not_equals);

    // Characters
    let c1 = 'a';
    let c2 = '3';
    let c3 = '+';
    let c4 = '\u{288A}';
    let c5 = '\"';
    println!("The value of c1 = {}, c2 = {}, c3 = {}, c4 = {}, c5 = {}", c1, c2, c3, c4, c5);

    let my_age = 40;
    let my_son_age = 35;
    println!("My age is {} and my son age is {}", my_age, my_son_age);

    let x1 = 40;
    let mut x2 = x1;
    x2 = x1 - 2;
    println!("My age is {} and my son age is {}", x1, x2);

    // let mut x1 = 40;
    // let x2;
    // x1 = x1 * 3;
    // x2 = x1 - 2;
    // println!("My age is {} and my son age is {}", x1, x2);

}


