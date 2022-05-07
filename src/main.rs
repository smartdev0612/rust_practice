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