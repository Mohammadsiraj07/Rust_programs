/*Write a program to implement the Scope and Shadowing*/
fn main() {
    let x = 50;
    {
        let x = 100; 
        println!("Inside block, x = {}", x);
    }
    println!("Outside block, x = {}", x);
}
