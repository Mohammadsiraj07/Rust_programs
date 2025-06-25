/*Write a program to create different types of constants print it in the output*/
const PI: f64 = 3.14159;
const APP_NAME: &str = "RustApp";
const IS_ENABLED: bool = true;

fn main() {
    println!("Numeric Constant: {}", PI);
    println!("String Constant: {}", APP_NAME);
    println!("Boolean Constant: {}", IS_ENABLED);
}
