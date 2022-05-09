
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
