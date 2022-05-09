
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
use std::{collections::HashMap, vec};
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
*/

//------------------------------------
// Closures
//  - A quick recap
//  - Borrow by immutable reference
//  - Borrow by mutable reference
//  - Moving of a value into a closure
//------------------------------------

/*
fn main() {
    let some_closure_1 = |x:u32| -> u32 {x + 1};
    let some_closure_2 = |x| {x + 1};
    let some_closure_3 = |x| x + 1;
}


fn main() {
    let mut vec_1 = vec![1,2,3];
    let mut some_closure = ||{
        // println!("Vec1: {:?}", vec_1); // infered to be used as immutable reference
        // vec_1.push(35);
        let vec_2 = vec_1;
    };

    some_closure();
    //vec_1[1] = 15;
    println!("Vec 1: {:?}", vec_1);
    println!("Vec 2: {:?}", vec_2);
} */

//------------------------------------
// Function Types
//  - Basic syntax and use
//  - Borrow by immutable reference
//  - Borrow by mutable reference
//  - Moving of a value into a closure
//  - Inference of the output
//  - Passing a closure as a function
//  - Reference to same variable
//------------------------------------

/*
fn main() {
    let mut f = max;
    println!("the maximum of the two values is {}", f(2, 3));
}

fn max(x: i32, y: i32) -> i32 {
    if x > y {
        x
    } else {
        y
    }
}

fn min(x:i32, y:i32) -> i32 {
    if x < y {
        x
    } else {
        y
    }
} 

fn prints_name(name: &str) {
    println!("The name is {}", name);
}

fn prints_full_info(f: fn(&str), some_name: &str, age: i32) {
    f(some_name);
    println!(" and my age is {}", age);
}

fn main() {
    let (my_name, my_age) = (String::from("Nouman"), 40);
    prints_full_info(prints_name, &my_name, my_age);
}


fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg:i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
}
*/

//------------------------------------
// Iterators
//  - Basics of iterators
//  - some useful functions for iterators
//  - Common statistics related functions
//  - Modifying and collecting values
//------------------------------------

/*
fn main() {
    let some_vec = vec![1,2,3,4,5,6,7,8];
    let mut iter = some_vec.iter();

    println!("The iterator : {:?}", iter);
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    

    let a:Vec<u32> = vec![0,1,2,3,4,5,6,7,8];
    let mut check = a.iter().any(|&x| x > 0);
    println!("The value of the any function is {}", check);

    let check = a.iter().all(|&x| x >= 0);
    println!("The value of the all function is {}", check);

    let check = a.iter().find(|&&x| x > 0);
    println!("The value of the function is {}", check.unwrap());

    let check = a.iter().position(|&x| x > 4);
    println!("The value of the function position is {}", check.unwrap());

    let check = a.iter().rposition(|&x|x > 4);
    println!("The value of the functon rpostion is {}", check.unwrap());

    let check = a.iter().max();
    println!("The value of the function max is {}", check.unwrap());

    let check = a.iter().min();
    println!("The value of the function min is {}", check.unwrap());

    let mut iter = a.iter().rev();
    println!("The result of applying the rev function {:?}", iter.next());

    let a = vec![0,1,2,3,4,5,6,7];
    let filtered_values = a.iter().filter(|&x| *x >= 5).collect::<Vec<&u32>>();
    println!("{:?}", filtered_values);

    let b = a.clone();
    let filtered_values = a.into_iter().filter(|x| *x >= 5).collect::<Vec<u32>>();
    println!("{:?}", filtered_values);

    let mapped_values = b.iter().map(|x| 2 * *x).collect::<Vec<u32>>();
    println!("{:?}", mapped_values);

    let mapped_values = b.iter().map(|x| 2 * *x).filter(|x| *x > 10).collect::<Vec<u32>>();
    println!("{:?}", mapped_values);
}
*/

// Find the sum of naturarl numbers using iterators
/*
fn main() {
    
    let mut n = String::new();
    std::io::stdin()
    .read_line(&mut n)
    .expect("failed to read input.");
    let n: u32 = n.trim().parse().expect("invalid input");
    
    let divisible_by_3_5 =  (1..n).into_iter().filter(|&x| x % 3 == 0 || x % 5 ==0 ).collect::<Vec<u32>>();   
    println!("{:?}", divisible_by_3_5);   
    println!("{:?}", divisible_by_3_5.iter().sum::<u32>());  
      
}
*/

// Compute intersection and union
fn main() 
{
    let mut vec_1: Vec<u32> = vec![5,4,3,6,9]; 
    let mut vec_2: Vec<u32> = vec![5,8,6,4,10,15,20,21]; 

   
    let intersect = intersection(&vec_1, &vec_2); 
    println!("\n\n The intersection of the two sets is {:?}",intersect); 

    let union_set = union(&mut vec_1, &mut vec_2, &intersect); 
    println!("\n\n The union of the set is  {:?} \n\n",union_set); 
 
}


fn intersection (first_set:&Vec<u32>, second_set:&Vec<u32>) -> Vec<u32>{
    let mut common:Vec<u32> = Vec::new(); 

    for i in first_set{
        let val = second_set.iter().find(|&x| x == i); 
        match val { 
            Some(common_val) =>  common.push(*val.unwrap()), 

            None => print!(""), 

        } 
    }
    common
}


fn union<'a> (first_set:&'a mut Vec<u32>, second_set:&'a mut Vec<u32>, common:&'a Vec<u32>) -> Vec<&'a u32>{
    
    for i in common{
        let position_of_common = first_set.iter().position(|&x| {x == *i});  
        first_set.remove(position_of_common.unwrap()); 

        let position_of_common = second_set.iter().position(|&x| {x == *i});  
        second_set.remove(position_of_common.unwrap());  
    }
    let union_set  = first_set.iter().chain(second_set.iter()).chain(common.iter()).collect::<Vec<_>>(); 
    union_set
    
}




