fn main() {
    // =>> Data Types <<==
    
    // ==> Integers <==
    // |----------------------------|
    // | lenght | signed | unsigned |
    // |----------------------------|
    // | 8bit   | i8     | u8       |
    // |----------------------------|
    // | 16bit  | i16    | u16      |
    // |----------------------------|
    // | 32bit  | i32    | u32      |
    // |----------------------------|
    // | 64bit  | i64    | u64      |
    // |----------------------------|
    // | 128bit | i128   | u128     |
    // |----------------------------|
    
    /*  The isize and usize types depend 
        on the kind of computer your program 
        is running on. The 64-bit type is used 
        on a 64-bit architecture, and the 32-bit 
        type on a 32-bit architecture. If you don't 
        specify the type for an integer, and the 
        system can't infer the type, it assigns the 
        i32 type (a 32-bit signed integer) by default.
    */

    let number: i32 = 14;
    println!("i32: {}", number);

    /*
        Rust has two floating-point data types for 
        decimal values: f32 (32 bits) and f64 (64 bits). 
        The default floating-point type is f64. 
        On modern CPUs, the f64 type is roughly the 
        same speed as the f32 type, but it has greater precision.   
    */

    let number_64 = 4.0;
    println!("number_64: {}", number_64);
    let number_32: f32 = 5.0;
    println!("number_32: {}", number_32);

    // ==> operations <==
    println!("1 + 2 = {}", 1u32 + 2);
    println!("8 - 5 = {}", 8i32 - 5);
    println!("15 * 3 = {}", 15 * 3);
    
    println!("9 / 2 = {}", 9u32 / 2);
    println!("9.0 / 2.0 = {}", 9.0 / 2.0);

    /*
        When we call the println function, we add the data type 
        suffix to each literal number to inform Rust about the 
        data type. The syntax 1u32 tells the compiler the value 
        is the number 1 and to interpret the value as an unsigned 
        32-bit integer.   If we don't provide type annotations, 
        Rust tries to infer the type from the context. 
        When the context is ambiguous, it assigns the i32 type 
        (a 32-bit signed integer) by default.
    */

    // ==> booleans <==
    let is_bigger = 1 > 4;
    println!("is 1 > 4? {}", is_bigger);
}
