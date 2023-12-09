pub fn main() {
    let a: i32 = 5;
    let b: i32 = 1000;
    let result: Result<i32, String> = add(a, b, 100);
    let sum: i32 = match result {
        Ok(value) => value,
        Err(message) => {
            println!("Error: {}", message);
            return;
        }
    };
    println!("{} + {} = {}", a, b, sum);
}

fn add(a: i32, b: i32, max: i32) -> Result<i32, String> {
    let sum = a + b;
    if sum > max {
        Err(format!("Sum {} is greater than max {}", sum, max))
    } else {
        Ok(sum)
    }
}
