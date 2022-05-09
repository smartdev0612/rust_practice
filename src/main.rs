use std::io::Read;

fn main() {
    // if conditional
    let some_number = 40;
    if some_number < 50 {
        println!("The number is less than 50");
    }

    // loops
    /* let my_number = 5;
    let mut guess = false;

    println!("Guess my number which is between 1 and 20");

    while guess != true {
        let mut number = String::new();
        std::io::stdin().read_line(&mut number).expect("failed to read input");
        let number:u8 = number.trim().parse().expect("Invalid input");

        if my_number == number {
            println!("You guessed the number correctly");
            guess = true;
        } else {
            println!("Please try again")
        }
    } 

    println!("Enter a number and I will tell you the next number after your number which is divisable by both 2 and 5");
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).expect("failed to read input");
    let mut number:u8 = number.trim().parse().expect("Invalid input");

    while (number % 2 == 0 && number % 5 == 0) != true {
        number = number + 1;
    }
    println!("The number after your number which is divisable by 2 and 5 is {}", number);
    */

    // For loops
    /*
    let mut some_vec = vec![45, 30, 85, 90, 15, 23, 98];
    for i in some_vec.iter() {
        println!("{}", i);
    }

    println!("{:?}", some_vec); 
    

    let mut some_vec = vec![45, 30, 85, 90, 15, 23, 98];
    for i in some_vec.iter_mut() {
        *i += 5;
        println!("{}", i);
    }
    println!("{:?}", some_vec);
    */

    // Break and Continue
    /*
    let mut var = 100;
    loop {
        var = var - 1;
        if var % 13 == 0 {
            break;
        }
    }
    println!("The highest number lesser than the given number divisable by 13 is {}", var);
    

    let mut var = 0;
    let mut count = 0;
    let req_number = loop {
        var = var + 1;
        if var % 5 == 0 && var % 3 == 0 {
            println!("the number which is divisable by both 3 and 5 is {}\n", var);
            count = count + 1;
            if count == 3 {
                break var;
            } else {
                continue;
            }
        }
        println!("{}", var);
    };
    println!("The required third highest number is {}", req_number);
    */

    // 1. Calculate the difference
    /*
    println!("Please enter the number:");
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).expect("failed to read input");
    let number:u8 = number.trim().parse().expect("Invalid input");

    let mut sum = 0;
    let mut sum_of_square = 0;
    
    for n in 1..number + 1 {
        println!("{}", n);
        sum += n;
        sum_of_square += (n as i32).pow(2);
    }

    println!("Square of sum is {}, and sum of square is {}", (sum as i32).pow(2), sum_of_square);
    println!("The difference is {}", (sum as i32).pow(2) - sum_of_square);
    */

    // 2. Sum of natural numbers
    /*
    println!("Please enter the number:");
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).expect("failed to read input");
    let number:u8 = number.trim().parse().expect("Invalid input");

    let mut multiples = vec![];
    let mut sum = 0;

    for n in 1..number {
        if n % 3 == 0 {
            multiples.push(n);
        } else if n % 3 == 0 && n % 5 == 0 {
            continue;
        } else if n % 5 == 0 {
            multiples.push(n);
        }
    }

    println!("Multiples: {:?}", multiples);

    for num in multiples {
        sum += num;
    }

    println!("Sum: {}", sum);
    */

    // 3. 
    /*
    let cars = production_rate_per_hour(1, 4);
    let cars2 = cars_produced_per_minutes(4, 4);
    println!("{} {}", cars, cars2);
    */

    // 4. Palindrome
    //println!("{}", palindrome("ele"));

    // 5. 

    // Stack
    /*
    println!("let us first create a stack for our use");
    println!("Please mention the size of the stack");

    let size_stack = input();
    let mut stack = new_stack(size_stack as usize);

    loop {
        println!("\n\n *** Menu *** \n");
        println!("1. Push \n 2. Pop \n 3. Display\n 4. Size\n 5. Exit");
        println!("\n Enter your choice: ");

        let choice = input();
        match choice {
            1 => {
                println!("Enter the value to be inserted: ");
                let item = input();
                push(&mut stack, item, size_stack as usize);
            },
            2 => println!("The element which is poped is {:?}", pop(&mut stack)),
            3 => println!("The elements are {:?}", stack),
            4 => println!("The size of the stack is {}", size(&stack)),
            5 => break,
            _ => println!("\n Wrong selection!!! Try again!!!"),
        }
        println!("Do you want to continue 1 = Yes / 0 = No");
        let status = input();

        if status == 1 {
            continue;
        } else {
            break;
        }
    }
    */

    /*
    let input_string = String::from("Welcome to Rust");
    let size_stack = input_string.len();
    let mut stack = new_stack(size_stack);
    let mut rev_string = String::new();

    for i in input_string.chars() {
        push(&mut stack, i, size_stack);
    }

    for i in 0..size(&stack) {
        rev_string.push(pop(&mut stack).unwrap());
    }

    println!("The input string is {:?}", input_string);
    println!("The reverse of the string is {:?}", rev_string);
    */

    // Traits
    /*
    let person1 = Person {
        name: String::from("Nouman Azam"),
        citizenship: String::from("Pakistan"),
        age: 40,
        gender: 'M',
        salary: 40_000,
    };

    let student1 = Student {
        name_std: String::from("Affan Azam"),
        age: 15,
        sex: 'M',
        country: String::from("USA"),
    };

    println!("The basic info include {:?}", person1.info());
    println!("The basic info for the student is {:?}", student1.info());
    

    let c1 = Circle{
        radius: 3.2,
    };

    let r1 = Rectangle {
        width: 5.0,
        length: 4.0
    };

    c1.area();
    c1.parameter();

    r1.area();
    r1.parameter();
    */

    let my_data = Data {
        some_data: vec![5, 6, 7, 8, 9, 10,3, 5, 4],
    };

    println!("The mean of the data is {}", my_data.mean());
    println!("The variance of the data is {}", my_data.variance());
}

struct Data {
    some_data: Vec<i32>,
}

struct Circle {
    radius: f32,
}

struct Rectangle {
    length: f32,
    width: f32,
}

trait BasicStats{
    fn mean(&self) -> f32;
    fn variance(&self) -> f32;
}

impl BasicStats for Data {
    fn mean(&self) -> f32 {
        let mut sum = 0;
        for i in self.some_data.iter() {
            sum = sum + i;
        }
        sum as f32 / self.some_data.len() as f32
    }

    fn variance(&self) -> f32 {
        let mu = self.mean();
        let mut sum_sqared_diff = 0.0;
        for i in self.some_data.iter() {
            sum_sqared_diff += (*i as f32 - mu) * (*i as f32 - mu);
        }
        sum_sqared_diff / self.some_data.len() as f32
    }
}

trait General_info {
    fn area(&self){
        println!("I am not implemented for this type");
    }

    fn parameter(&self);
}

impl General_info for Circle {
    // fn area(&self) {
    //     let area_of_circle = 3.14 * (self.radius * self.radius);
    //     println!("The area of the circle is {}", area_of_circle);
    // }

    fn parameter(&self) {
        let circumference = 2.0 * 3.14 * self.radius;
        println!("the circumference of the circle is {}", circumference);
    }
}

impl General_info for Rectangle {
    // fn area(&self) {
    //     let area_of_rect = self.length * self.width;
    //     println!("The area of the rectangle is {}", area_of_rect);
    // }

    fn parameter(&self) {
        let parameter_rect = 2.0 * (self.length + self.width);
        println!("The area of the reactangle is {}", parameter_rect);
    }
}

/*
struct Person {
    citizenship: String,
    name: String,
    age: u8,
    gender: char,
    salary: i32,
}

struct Student {
    name_std: String,
    age: u8,
    sex: char,
    country: String,
}

trait General_info {
    fn info(&self) -> (&str, u8, char);
    fn country_info(&self) -> &str;
}

impl General_info for Person {
    fn info(&self) -> (&str, u8, char) {
        (&(self.name), self.age, self.gender)
    }

    fn country_info(&self) -> &str {
        &self.name
    }
}

impl General_info for Student {
    fn info(&self) -> (&str, u8, char) {
        (&self.name_std, self.age, self.sex)
    }

    fn country_info(&self) -> &str {
        &self.name_std
    }
}
*/

fn new_stack(maxsize: usize) -> Vec<char> {
    let vec: Vec<char> = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<char>) -> Option<char> {
    let poped_value = stack.pop();
    // println!("The poped value is {:?}", poped_value);
    poped_value
}

fn push(stack: &mut Vec<char>, item: char, maxsize: usize) {
    if stack.len() == maxsize {
        // println!("Can't add more elements");
    } else {
        stack.push(item);
        // println!("Stack: {:?}", stack);
    }
}

fn size(stack: &Vec<char>) -> usize {
    stack.len()
}

fn input() -> u32 {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("Failed to read input");
    let n:u32 = n.trim().parse().expect("Invalid input");
    n
}

fn production_rate_per_hour(number_of_hours: i32, speed: i32) -> i32 {
    match speed {
        1..=4 => number_of_hours * speed * 221,
        5..=8 => number_of_hours * speed * 221 * 90 / 100,
        9 | 10 => number_of_hours * speed * 221 * 77 / 100,
        _ => 0 
    }
}

fn cars_produced_per_minutes(hours: i32, speed: i32) -> i32 {
    match speed {
        1..=4 => speed * 221 / 60,
        5..=8 => speed * 221 * 90 / 100 / 60,
        9 | 10 => speed * 221 * 77 / 100 / 60,
        _ => 0 
    }
}

fn palindrome(word: &str) -> bool {
    let reversed_word = word
        .chars()
        .rev()
        .collect::<String>();
    word == reversed_word
}