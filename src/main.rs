
/*
#[derive(Debug)]
enum Value {
    Integer(i32),
    Float(f32),
}

// Generics
fn squarei32(x:i32) -> i32 {
    x * x
}

fn squaref32(x:f32) -> f32 {
    x * x
}


fn square<T: std::ops::Mul<Output = T> + Copy> (x:T) -> T {
    x * x
}

fn main() {
    println!("The square of the number is {}", square(5));
    println!("The square of the number is {}", square(5.5));
}
*/

// Generics
/*
struct Point<T, U> {
    x: T,
    y: U,
}

impl <T, U> Point<T, U>
where T: std::fmt::Debug, U: std::fmt::Debug {
    fn printing(&self) {
        println!("The value of the points are {:?}, {:?}", self.x, self.y)
    }
}

fn main() {
    let p1 = Point {
        x: 5,
        y: 5
    };

    let p2 = Point {
        x: 1.0,
        y: 4.0,
    };

    let p3 = Point {
        x: 5,
        y: 5.0,
    };

    p1.printing();
}
*/

// Option Enum

// enum Option<T> {
//     None,
//     Some(T),
// }

/*
fn main() {
    let mut disease:Option<String> = None;
    disease = Some(String::from("Diabetes"));

    match disease {
        Some(disease_name) => println!("You have the disease of {}", disease_name),
        None => println!("You do not have any disease"),
    }
}
*/

/*
fn main() {
    let s1:Option<&str> = Some("Some String");
    println!("The value of s1 is {:?}\n the value of s1 itself is {:?}", s1, s1.unwrap());

    let f1:Option<f64> = Some(10.54);
    let mut f2 = 16.5;
    f2 = f2 + f1.unwrap();

    println!("The value of the sum is {}", f2);

    let v1:Option<Vec<i32>> = Some(vec![0,1,2,3]);

    let p1 = Person {
        name:String::from("Nouman"),
        age: 32,
    };
    let some_one:Option<Person> = Some(p1);
}

struct Person {
    name: String,
    age: i32,
}


fn square(num: Option<i32>) -> Option<i32> {
    match num {
        Some(number) => Some(number * number),
        None => None,
    }
}

fn main() {
    let number = Some(6);
    if square(number) != None {
        println!("the result of the square is {:?}", square(Some(6)).unwrap());
    } else {
        println!("we do not have any values");
    }
}
*/

// Result Enum
/*fn division(divident: f64, divisor:f64) -> Result<f64, String> {
    /*if divisor == 0.0 {
        Err(String::from("Error: Division by zero"))
    } else {
        Ok(divident / divisor)
    }*/

    match divisor {
        0. => Err(String::from("Error: Division by zero")),
        _ => Ok(divident / divisor)
    }
}

fn main() {
    println!("{:?}", division(9.0, 3.0));
    println!("{:?}", division(4.0, 0.0));
    println!("{:?}", division(0.0, 2.0));
}*/

/*
fn main() {
    let some_vec = vec![5,5,4,3,2,3,7,8];
    let result = match some_vec.get(5) {
        Some(a) => Ok(a),
        None => Err("the value does not exist"), 
    };
    println!("The value of the result is {:?}", result);
}
*/

// Hash Maps
use std::collections::HashMap;
/*fn main() {
    let mut person:HashMap<&str, i32> = HashMap::new();
    person.insert("Nouman", 40);
    person.insert("Kamran", 44);
    person.insert("Shahid", 55);

    println!("The age is {:?}", person.get("Nouman").unwrap());

    if person.contains_key("Nouman") {
        println!("The value exist");
    } else {
        println!("The value does not exist");
    }

    match person.get("Nouman") {
        Some(Value) => println!("The value exists {}", Value),
        None => println!("The value does not exist"),
    }

    for (name, age) in &person {
        println!("the person {} has an age of {}", name, age);
    }
}*/

/*
fn main() {
    let mut likes:HashMap<&str, &str> = HashMap::new();
    /*likes.insert("Nouman", "apple");
    likes.insert("Nouman", "mango");
    println!("The fruit which is being liked is {:?}", likes);
    */

    likes.entry("Nouman").or_insert("apple");
    likes.entry("Nouman").or_insert("Mango");
    println!("The fruit which is being liked is {:?}", likes);
}
*/

/*
fn main() {
    let some_vec:Vec<i32> = vec![4,5,6,7,7,4,3,5,6];
    let mut freq_vec:HashMap<i32, u32> = HashMap::new();
    for i in &some_vec {
        let freq: &mut u32 = freq_vec.entry(*i).or_insert(0);
        *freq += 1;
    }
    println!("{:?}", freq_vec);
}
*/

//------------------------------------
// Lifetimes
//  - Dangling Reference
//  - Undetermined lifetimes
//  - Generic Lifetimes Parameters
//  - GLP typically needed with outputs from functions that are reference
//  - GLP with multiple variables
//  - GLP and structures
//  - Reference to same variable
//------------------------------------

/*
fn main() {
    let i:&i32;
    {
        let j = 5;
        i = &j;
    }
    println!("The value of i = {}", i);
} */

/*
fn main() {
    let some_int = 10;
    let additional_int = some_fn(&some_int);
    println!("{}", additional_int);
}

fn some_fn(i:&i32) -> &i32 {
    &i
} */

/*
fn main() {
    let int1 = 5;
    let int2 = 10;
}

fn greater(i:&i32, j:&i32) -> &i32 {
    if i > j{
        i
    } else {
        j
    }
}
*/

/*
fn main() {
    let s_1 = "Hello";
    let v;
    {
        let s_2 = String::from("world");
        v = some_fn(s_1, s_2.as_str());
    }
    println!("{}", v);
}

fn some_fn<'a, 'b>(first_str:&'a str, second_str:&'b str) -> &'a str {
    first_str
}
*/

/* 
fn main() {
    let int1 = 5;
    let int2 = 10;
    let result = greater(&int1, int2);
}

fn greater<'a>(i:&'a i32, j:i32) -> &'a i32 {
    i
} */

/*
fn main() {
    let int1 = 5;
    {
        let int2 = 10;
        let result = greater(&int1, &int2);
        println!("the larger value is {}", result);
    }
}

fn greater<'a, 'b>(i:&'a i32, j:&'a i32) -> &'a i32 {
    if i > j {
        i
    } else {
        j
    }
}
*/

/* 
struct Person<'a> {
    name: &'a str,
    age: i32,
}

fn main() {
    let first_name = "Nouman";
    let mut nouman = Person {
        name: &first_name,
        age: 40,
    };

    {
        let last_name = String::from("Azam");
        nouman.name = &last_name;
    }
    println!("the name of the person is {} and his age is {}", nouman.name, nouman.age);
} */

/*
fn main() {
    let some_vec = vec![5, 7, 8, 9, 7, 9, 5, 2];
    let return_vec = use_vec(&some_vec, &some_vec);
}

fn use_vec<'a>(vec1: &'a [i32], vec2: &'a [i32]) -> &'a [i32] {
    if 3 > 5 {
        vec1
    } else {
        vec2
    }
} */

//------------------------------------
// Closures
//  - Basic syntax
//  - Closure with inputs
//  - Same variable for different closure
//  - Ownership rules and closures
//  - Inference of the output
//  - Passing a closure as a function
//  - Reference to same variable
//------------------------------------

/*
fn main() {
    let x = 5;
    let square = |num:i32| println!("the square of {} is {}", num, num * num);
    let square = |num:i32| println!("the cube of {} is {}", num, num * num * num);
    square(x);

    let y = 15;
    square(y);
} 

fn main() {
    let print_user_age = |general_info:String, name:&str, age:i32| println!("{} \n\t {} : {}", general_info, name, age);
    let general_info = String::from("The details are");
    let (person_name, person_age) = (String::from("Nouman"), 51);

    print_user_age(general_info, &person_name, person_age);
    println!("The variable has been moved {}", general_info);
}


fn main() {
    let square = |num| num * num;

    let x = 5;
    square(x);

    let y = 105.5;
    square(y);
}
*/

fn main() {
    let division_status  = |y:f32| {if y != 0.0 {true} else {false}};
    division(5.0, 10.0, division_status);
    division(54.0, 0.0, division_status);
}

fn division<F: Fn(f32) -> bool>(x:f32, y:f32, f: F) {
    if f(y) == true {
        println!("The division result is {}", x/y);
    } else {
        println!("Division is not possible");
    }
}













