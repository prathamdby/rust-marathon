fn main() {
    println!("{}", is_even(20));
}

fn is_even(num: i32) -> bool {
    // Bitwise operation
    // & 1 is faster than doing % 2
    return num & 1 == 0;
}
