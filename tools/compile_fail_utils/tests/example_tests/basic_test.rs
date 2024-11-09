//! A basic example of a compilation failure test.
//! Use this as a template or for help with syntax.

// Compiler warnings also need to be annotated.
#![allow(unused_variables, reason = "The variables are for basic demonstration purposes and don't need to be used.")]

fn bad_moves() {
    let x = String::new();
    // Help diagnostics need to be annotated
    let y = x;
    //~^ HELP: consider cloning

    // We expect a failure on this line
    println!("{x}"); //~ ERROR: borrow


    let x = String::new();
    // We expect the help message to mention cloning.
    //~v HELP: consider cloning
    let y = x;

    // Check error message using a regex
    println!("{x}");
    //~^ ERROR: /(move)|(borrow)/
}
