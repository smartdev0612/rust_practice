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

    // function
    basic_fn();

    function_with_inputs("Nouman", 40_000);

    let full_name = "Kamran";
    let salary = 50_000;

    function_with_inputs(full_name, salary);

    let answer = function_with_inputs_outputs(10, 15);
    println!("The answer of multiplication is {}", answer);

    let (multi, addi, subt) = function_with_inputs_multiple_outputs(10, 15);
    println!("Mulitplication: {}, addition = {}, subtraction = {}", multi, addi, subt);

    let results = function_with_inputs_multiple_outputs(10, 15);
    println!("Multiplication = {}, addition = {}, subtraction = {}", results.0, results.1, results.2);

    let full_name = {
        let first = "Nouman";
        let last_name = "Azam";
        format!("{} {}", first, last_name)
    };

    println!("My full name is {}", full_name);

    // Inputs from users
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input.");

    let n:f64 = n.trim().parse().expect("invalid input");
    println!("{:?}", n);

    // Practice
    println!("Input the width of a rectangle");
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input");
    let width: u32 = n.trim().parse().expect("invalid input");

    println!("Input the height of a rectangle");

    std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input");
    let length: u32 = n.trim().parse().expect("invalid input");

    let resultant_area = {
        area(width, length)
    };

    println!("The area of is {}", resultant_area);
}

fn area(length:u32, width:u32) -> u32 {
    length * width
}

fn basic_fn() {
    println!("This is  a basic function");
}

fn function_with_inputs(name: &str, salary: i32) {
    println!("The name is {} and the salary is {}", name, salary);
}

fn function_with_inputs_outputs(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn function_with_inputs_multiple_outputs(num1: i32, num2: i32) -> (i32, i32, i32) {
    (num1 * num2, num1 + num2, num1 - num2)
}

struct Calculator {
    balance: f64
}

trait Operator {
    fn add(num1: f64, num2: f64) -> f64;
    fn sub(num1: f64, num2: f64) -> f64;
    fn div(num1: f64, num2: f64) -> f64;
    fn mul(num1: f64, num2: f64) -> f64;
}

impl Operator for Calculator {
    fn add(num1: f64, num2: f64) -> f64 {
        num1 + num2
    }

    fn sub(num1: f64, num2: f64) -> f64 {
        num1 - num2
    }

    fn div(num1: f64, num2: f64) -> f64 {
        num1 / num2
    }

    fn mul(num1: f64, num2: f64) -> f64 {
        num1 * num2
    }
}
