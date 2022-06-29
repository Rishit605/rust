fn main() {
    // Mutating in Rust
    println!("This is Mutaition of a variable.");
    
    let mut x = 4; // variables are immutable and can be mutated with 'mut' syntax in prefix of the variable
    println!("x is: {}", x);

    x = 0;
    println!("x is: {}\n", x);

    // Overwriting in Rust
    println!("This is Overriding  a variable.");

    let  y = 3;
    println!("y is: {}", y);

    let y = 9;
    println!("y is: {}\n", y);
    
    // {"""
    // Name Shadowing
    // """}

    println!("This is name Shadfowing or Scoping.");

    let o = 2;
    println!("o is: {}", o);
    
    {
        let o = 7;
        println!("o is: {}", o);

        let o = o + 8;
        println!("o is: {}", o);
    }

    let o = 6;
    println!("o is: {}\n", o);
    
    // Data Types: Scalar and Compound are the 2 data types.

    // SCALAR DATA TYPES:
    let x: i32 = 2; // Explicitly assinging an integer data type to a variable
                   // Like "'i8', 'i16', 'i32', 'i64', 'i128'" [where i is integer and 8/16/32/64/128 are the bits]
                   // Range of values represented bu i8 will be from [-2^7 to 2^7 - 1]

    println!("x -> : {} is a integer Data Type", x);


    let y: u32 = 4; // Explicitly assinging a variable to an un-signed integer data type, i.e no postive or negative sign would be accepted by the varible on an intger.
                   // Like the same as the above integer data types, "'u8', 'u16', 'u32', 'u64', 'u128'" [where u is an un-assigned integer and 8/16/32/64/128 are the bits]
                   // Range of values represented by i8 will be from [0 to 2^8 - 1]

    println!("y -> : {} is a unassigned integer Data Type", y);


    let p: f32 = 6.3; // Explicitly assinging a variable to an un-signed integer data type, i.e no postive or negative sign would be accepted by the varible on an intger.
                      // Like "'f32', 'f64'" [where f is an floating point and 8/16/32/64/128 are the bits]

    println!("p -> : {} is a floating value Data Type", p);


    let h: bool = true; // Explicitly assinging a variable to an true or false Boolean data type, i.e a Ture or False values, an Boolean Value.
                        // Range of values represented by [True/False or 1/0 respectively].

    println!("h -> : {} is Boolean Data Type", h);


    let r: char = ';'; // Explicitly assinging a variable to any character value being any alphabet or symbol.

    println!("r -> {} is a char Data Type\n", r);
}
