// use std::fs::read_to_string;
use chrono::{Local, Utc};

fn main() {
    // println!("{}", is_even(20));
    // println!("{}", fib(4));

    // let my_str = String::from("Hello, world!");
    // let length = get_string_len(my_str);
    // println!("{}", length);

    // let user = User {
    //     first_name: String::from("Pratham"),
    //     last_name: String::from("Dubey"),
    //     age: 20,
    // };

    // println!("{}", user.first_name);
    // println!("{}", user.last_name);
    // println!("{}", user.get_age());
    // println!("{}", User::debug());

    // let my_circle = Shape::Circle(5.0);
    // print_area(my_circle);
    // let my_rect = Shape::Rectangle(2.0, 4.0);
    // print_area(my_rect);

    // let index = find_first_a(String::from("Pratham"));

    // match index {
    //     Some(value) => println!("The index is {}", value),
    //     None => println!("The character 'a' not found"),
    // }

    // let file_result = read_to_string("file.txt");

    // match file_result {
    //     Ok(content) => println!("{:?}", content),
    //     Err(err) => println!("Following error occured: {:?}", err),
    // }

    let now = Utc::now();
    println!("Current date and time in UTC is: {}", now);

    let formatted_now = now.format("%Y-%m-%d %H:%M:%S");
    println!("Formatted date and time: {}", formatted_now);

    let local = Local::now();
    println!("Current date and time in Local is: {}", local);
}

// enum Shape {
//     Rectangle(f64, f64),
//     Circle(f64),
// }

// fn print_area(shape: Shape) {
//     let area = match shape {
//         Shape::Circle(r) => 3.14 * (r * r),
//         Shape::Rectangle(l, b) => l * b,
//     };
//     println!("{}", area);
// }

// fn find_first_a(s: String) -> Option<i32> {
//     for (i, char) in s.chars().enumerate() {
//         if char == 'a' {
//             return Some(i as i32);
//         }
//     }

//     return None;
// }

// struct User {
//     first_name: String,
//     last_name: String,
//     age: i32,
// }

// impl User {
//     fn get_age(&self) -> i32 {
//         self.age
//     }

//     fn debug() -> i32 {
//         return 1;
//     }
// }

// fn is_even(num: i32) -> bool {
//     // Bitwise operation
//     // & 1 is faster than doing % 2
//     return num & 1 == 0;
// }

// fn fib(num: i32) -> i32 {
//     let mut first: i32 = 0;
//     let mut second: i32 = 1;

//     if num == 0 {
//         return first;
//     }

//     if num == 1 {
//         return second;
//     }

//     // Here num - 1 or num - 2 ain't needed
//     for _ in 1..num {
//         let temp = second;
//         second += first;
//         first = temp;
//     }

//     return second;
// }

// fn get_string_len(str: String) -> usize {
//     // Implicit return here, no need to put "return" or ";"
//     str.chars().count()
// }
