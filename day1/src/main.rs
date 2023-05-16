fn main() {
    let count = factorial(5);
    println!("Factorial of 5 is {}", count);
}

fn factorial(num: u64) -> u64 {
    match num {
        0 => 1,
        _ => factorial(num - 1) * num,
    }
}
