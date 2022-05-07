fn main() {
    // Rust Ownership
    let mut x = 32;
    let mut y = x;
    println!("The value of x = {}, y = {}", x, y);

    let s1 = String::from("abc");
    let s2 = &s1;
    println!("The value of s1 = {} and the value of s2 = {}", s1, s2);

    let num_vec1:Vec<i32> = vec![5, 6, 9, 8, 7];
    let num_vec2 = &num_vec1;
    println!("The first vector is {:?} and the second one is {:?}", num_vec1, num_vec2);

    let num_vec2 = num_vec1.clone();
    println!("The first vector is {:?} and the second one is {:?}", num_vec1, num_vec2);

    {
        // scope 
        let my_name = String::from("Nouman Azam");
    }

    // println!("My name is {}", my_name);

    // Ownership and function
    let stack_num = 32;
    let mut heap_num = vec![4, 5, 6];
    stack_function(stack_num);
    println!("The stack variable is copied and the original value was {}", stack_num);

    heap_function(&mut heap_num);
    println!("the value of the vector outside the function is {:?}", heap_num);

    // Quiz
    let mut heap_num = vec![4, 5, 6];
    let ref1 = heap_num;
    let ref2 = &ref1;

    // let mut heap_num = vec![4, 5, 6];
    // let ref1 = &mut heap_num;
    // let ref2 = &mut heap_num;
    // println!("the first reference is {:?} and the second one is {:?}", ref1, ref2);

    // let mut heap_num = vec![4, 5, 6];
    // let ref1 = &heap_num;
    // let ref2 = &heap_num;
    // println!("The first references is {:?} and the second one is {:?}", ref1, ref2);

    // let mut heap_num = vec![4, 5, 6];
    // let ref1 = &heap_num;
    // let ref2 = &heap_num;
    // let ref3 = &mut heap_num;
    // println!("Immutable references are {:?} and {:?} and the mutalbe reference is {:?}", ref1, ref2, ref3);

    // let mut heap_num = vec![4, 5, 6];
    // let ref1 = &heap_num;
    // let ref2 = &heap_num;
    // println!("Immutable references are {:?} and {:?}", ref1, ref2);

    // let ref3 = &mut heap_num;

    let mut heap_num = vec![4, 5, 6];
    heap_num.push(86);
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    println!("Immutalbe references are {:?} and {:?}", ref1, ref2);

    heap_num.push(86);

    // String concatenation and ownership
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = s1 + &s2;
    println!("{} {}", s3, s2);

    let s1:String = String::from("this is me");
    let s2: &str = "myself";

    some_function(&s1,s2); 
    println!("{} {}",s1,s2);
}

fn some_function(a1: &String, a2: &str){
    println!("{} {}",a1,a2); 
}

fn stack_function(mut var:i32) {
    var = 46;
    println!("The copied value of the variable has been updated to {}", var);
}

fn heap_function(var: &mut Vec<i32>) {
    var.push(50);
    println!("The value of the vector inside the function is {:?}", var);
}