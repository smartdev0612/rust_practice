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

}

fn stack_function(mut var:i32) {
    var = 46;
    println!("The copied value of the variable has been updated to {}", var);
}

fn heap_function(var: &mut Vec<i32>) {
    var.push(50);
    println!("The value of the vector inside the function is {:?}", var);
}