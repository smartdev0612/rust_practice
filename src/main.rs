
#[derive(Debug)]
enum Value {
    Integer(i32),
    Float(f32),
}

fn main() {
    let some_val = vec![Value::Integer(12), Value::Float(15.5)];
    println!("The value of the integer is {:?} and the value of the float is {:?}", some_val[0], some_val[1]);

    for i in some_val.iter() {
        match i {
            Value::Integer(num) => println!("The vlaue is an integer with a value of {}", num),
            Value::Float(num) => println!("The value is a float with a value of {}", num)
        }
    }
}
