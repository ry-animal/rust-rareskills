// ### Exercise 1 - Ownership and Borrowing

// Copy/paste the following code to a Rust `main.rs` file

// - Explain why this code snippet does not work.
// - Give at least 2 ways to fix the issue (there are more than 2).

// ```rust
// fn function_1(var: String) {

//     println!("In function_1, variable is: {}", var);
// }

// fn main() {
//     let variable = String::from("Welcome to RustSkills");

//     function_1(variable);

//     println!("In main, variable is: {}", variable);
// }
// ```

// Now, replace the `String` variable with a scalar variable (`u32, i32, u64, i64, …`) and retest the same code snippet.

// - Why does it work?

// ownership is transferred. `variable` is no longer valid in main

// borrowing
fn function(var: &String) {
    println!("In function, variable is: {}", var);
}

fn main() {
    let variable = String::from("Welcome to RustSkills");

    function(&variable);

    println!("In main, variable is: {}", variable);
}

// clone
fn function2(var: String) {
    println!("In function2, variable is: {}", var);
}

fn main2() {
    let variable = String::from("Welcome to RustSkills");

    function2(variable.clone());

    println!("In main, variable is: {}", variable);
}
