fn main() {
    let mut x = 5;
    println!("Here is the value of x: {}", x);
    x = 6;
    println!("Here is the value of x: {}", x);

    /*
        You cannot mutate a constant.
        Const variables must be type annotated.
        Const variables can only be set to constant variables.
            The value must be known at run time.
    */

    const CONSTANT_VARIABLE: u32 = 1_000_000_000;
    println!("Here is a constant variable: {}", CONSTANT_VARIABLE);

    /*
        Shadowing:
    */

    let _y: i32 = 0; // this _y has been shadowed and it is still immutable
    let _y: i32 = 4;
    let _y: String = String::from("5");
}
