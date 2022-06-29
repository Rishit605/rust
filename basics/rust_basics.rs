use std::io; // Used for importing libraries

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
    
    // -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- //

    // COMPOUND DATA TYPES:
    let t: (i32, bool, char) = (1, true, 's'); // Explicitly assinging a variable to a tuple value whihc includes any integer, alphabet or symbol and boolean value.

    println!("t -> {} is Tuple Data Type", t.1); // "t.0/1/2" we can access the elements of the tuple individually.

    let arr: [i32; 2] = [1, 7]; // Explicitly assinging a variable to a array value which includes any integer, alphabet or symbol and boolean value.

    println!("arr -> {} is a Array Data Type", arr[1]); //Accessing elements in Array

    // USER INPUT:
    let mut inp = String::new();

    io::stdin().read_line(&mut inp).expect("Failed to read line");
    println!("{}", inp);

    // ARITHIMETICS:

    // ERRO THRWOING STATMENTS //
    // let mut x: u8 = 9; // 0 to 255
    // let mut y: i8 = 21; // -128 to 127

    // let mut z = x + y; // Will throw an error as the language does not know how to add different data type values.

    // let x: f32 = 9; // 0 to 255
    // let y: i8 = 21; // -128 to 127

    // let z = x + y; // Will throw an error as the language does not know how to add different data type values.

    let x: i8 = 9; 
    let y: i8 = 21;

    let mut z = x + y; // Will throw an error as the language does not know how to add different data type values.
    println!("z -> {} is the summation two variables with the same Data Types", z);

    z = x - y;
    println!("z -> {} is the subtraction two variables with the same Data Types", z);

    let x: f32 = 144.2;
    let y: f32 = 12.0;

    let z = x / y;
    println!("z -> {} is the division two variables with the same Data Types\n", z);

    // TYPE CASTING AND CONVERSION:
    
    let q = 255.0f32; // Default dtype would be f64 but with f32 just behind the value to tell the language to treat this value as f32 dtype.
    println!("q -> {} is not explicitly defined varible but is changed from f64(default) to f32", q);

    let q = 122_000 as i64;
    println!("q -> {} is not explicitly defined varible but is changed from f64(default) to f32", q);

    let q = 122_000 as i64;
    let w = 11_0 as i32;

    let i = q / (w as i64);  
    println!("i -> {} is not explicitly defined varible and w is cast to a different varible(i32), but is later defined as i64.", i);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("expected to read line");
    println!("{}", input);

    let int_inp: i64 = input.trim().parse().unwrap();
    println!("{}", int_inp + 2);
}
