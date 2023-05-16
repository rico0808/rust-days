fn main() {
    count(5);
}

fn count(num: i32) {
    let mut count = num;
    for item in 0..num {
        if item <= 0 {
            continue;
        }
        count *= item;
    }
    println!("count is {}", count)
}
