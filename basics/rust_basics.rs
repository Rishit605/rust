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
}
