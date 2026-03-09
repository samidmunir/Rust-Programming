/*
    Variables
    - A variable is a name assigned to a value in the program. Serves as an identifier to what context the variable represents/stores.
    - The Rust compiler will provide a warning for any unused variables.
    - The Rust compiler will also infer the type of the variable based on the value stored (if not explicitly specified by the developer).
    - We can declare unused variables by prepending the variable name with an underscore. This is so that the Rust compiler does not generate unused variable warnings.

    Interpolation
    - We can print variables to the terminal using the println!() macro, by placing String literals in "" and interpolating variables by using {}. After ending our String, we can pass variable names as arguments for each pair of curly braces. We can also pass variable names directly within the {} and not pass variable names as arguments to println!().

    Positional arguments to println!()
    - When we pass positional arguments to the println!() macro, Rust assigns numerical correlations/indices to each starting from 0.

    Mutability & immutability
    - Variables are immutable by default. This means they are incapable of change. Mutable means capable of change.
    - The mut keyword allows a variable to be mutable (or changed in the lifetime of the program).

    Rust error codes
    - The Rust compiler will provide error codes which we can copy and use the rustc --explain <code> command to allow the Rust compiler to help us understand they exact error. It will provide extensive documentation within the terminal.
    - We can also visit the Rust Error Codes Index online to view a list of all Rust error codes.

    Variable shadowing
    - This means we are redeclaring a variable. The original variable is replaced by the new one. The original one becomes invalid or over-shadowed.
        > same name
        > new type
        > different value
        > this is different from variable reassignment.

    Scopes & blocks

    Constants

    Type aliases

    Compiler directives
*/

fn main() {
    println!("\nVariables, Data Types, Mutability in Rust programming!");

    let lines_of_code: i32 = 253;
    println!("\nlines_of_code = {}", lines_of_code);
    let more_lines_of_code: i32 = 173;
    println!("more_lines_of_code = {}", more_lines_of_code);
    let total_lines_of_code = lines_of_code + more_lines_of_code;
    println!(
        "total_lines_of_code = {} + {} = {}",
        lines_of_code, more_lines_of_code, total_lines_of_code
    );

    let a = 3;
    let b = 1;
    let c = 2;
    println!("\n{1}, {2}, {0}", a, b, c);

    let _unused_var = 7;

    let mut tax_rate: f32 = 1.2573;
    println!("\ntax_rate: {}", tax_rate);
    tax_rate = 1.47892;
    println!("tax_rate: {}", tax_rate);

    /*
        For Rust Error Code example...
    */
    // let x = 0;
    // x = 1;

    let grams_of_protein: &str = "100.345";
    println!("\ngrams_of_protein: {} (&str)", grams_of_protein);
    let grams_of_protein = 100.345;
    println!("grams_of_protein: {} (f64)", grams_of_protein);
    let grams_of_protein = 100;
    println!("grams_of_protein: {} (i32)", grams_of_protein);
    let grams_of_protein = 95;
    println!("grams_of_protein: {} (i32)", grams_of_protein);
}
