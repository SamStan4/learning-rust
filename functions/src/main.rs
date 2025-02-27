fn main() {
    my_function();
    println!("1 + 1 = {}", add_numbers(1, 2));
    println!("6 / 2 = {}", divide_numbers(6, 2).unwrap());
}

fn my_function() {
    println!("Here is another function");
}

fn add_numbers(num_one: i32, num_two: i32) -> i32 {
    num_one + num_two
}

fn divide_numbers(num_one: i32, num_two: i32) -> Result<f32, String> {
    if num_two == 0 {
        Err("Divide by zero error".to_string())
    } else {
        let result = num_one / num_two;
        Ok(num_one as f32 / num_two as f32)
    }
}