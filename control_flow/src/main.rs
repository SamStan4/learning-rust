fn main() {
    let mut i = 0;
    let j = loop {
        if i == 10 {
            break i;
        }
        i += 1;
    };
    println!("Here is j: {}", j);
    let mut number = 1000;
    while number != 0 {
        println!("Here is number: {}", number);
        number -= 1;
    }
    for k in 1..=100 {
        println!("Here is k: {}", k);
    }
}
