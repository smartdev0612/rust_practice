fn main() {
    // Vectors
    let mut number_vec:Vec<i32> = vec![5, 6, 7, 8, 9, 10, 11, 12, 13, 16];
    println!("{}", number_vec[0]);
    number_vec[4] = 5;
    println!("{:?}", number_vec);

    let array_with_some_elements:Vec<i32> = vec![0;10];

    let mut string_array_1:Vec<&str> = vec!["apple", "tomato", "grapes"];
    string_array_1[0] = "Kamran Azam";

    let string_array_2:Vec<&str> = vec!["Unknown"; 6];

    let char_vec:Vec<char> = vec!['a', 'p', 'p', 'l', 'e'];

    let subset_vec = &number_vec[0..3];
    println!("the subset of the values of the array {:?}", subset_vec);

    println!("Elements in the array are {}", number_vec.len());

    let check_index = number_vec.get(2);
    println!("{:?}", check_index);

    number_vec.push(30);
    number_vec.push(31);
    println!("The value are the vector are {:?}", number_vec);

    number_vec.remove(5);
    println!("The value after removing are {:?}", number_vec);

    println!("The value of 10 exist in the array {}", number_vec.contains(&10));

}


