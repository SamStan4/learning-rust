fn main() {
    /*
        Scalar data types
    */

    // Integers

    let _a: i32 = 98_222;      // Decimal
    let _b: i32 = 0xFF;        // Hex
    let _c: i32 = 0o77;        // Octal
    let _d: i32 = 0b1111_0000; // Binary
    let _e: u8 = b'A';         // Byte u8 only

    // Floating point numbers

    let _f: f32 = 3.3;
    let _g: f64 = 6.9;

    // Booleans

    // Characters

    /*
        Compound types
    */

    let tup = ("lets code some rust", 64);
    let _tup_one = tup.0;
    let _tup_two = tup.1;
    let (_tup_one_two, _tup_two_two) = tup;

    let http_codes = [200, 400, 500];
    let _random_list = [0; 9];

    let _ok_status = http_codes[0];
}