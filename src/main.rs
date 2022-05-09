
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

fn main() {
    let some_vec:Vec<i32> = vec![4,5,6,7,7,4,3,5,6];
    let mut freq_vec:HashMap<i32, u32> = HashMap::new();
    for i in &some_vec {
        let freq: &mut u32 = freq_vec.entry(*i).or_insert(0);
        *freq += 1;
    }
    println!("{:?}", freq_vec);
}