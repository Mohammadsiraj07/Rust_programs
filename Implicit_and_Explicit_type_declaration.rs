/*Write a program to implement the following
a. Implicit type declaration
b. Explicit type declaration*/
fn main() {
    let a = 10.5;
    println!("Implicit: a = {} | type = f64", a);
    let b: i32 = 20;
    println!("Explicit: b = {} | type = i32", b);
}
