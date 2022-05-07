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
}
