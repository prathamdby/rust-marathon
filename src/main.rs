fn main() {
    // println!("{}", is_even(20));
    // println!("{}", fib(4));

    let my_str = String::from("Hello, world!");
    let length = get_string_len(my_str);
    println!("{}", length);
}

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

fn get_string_len(str: String) -> usize {
    // Implicit return here, no need to put "return" or ";"
    str.chars().count()
}
