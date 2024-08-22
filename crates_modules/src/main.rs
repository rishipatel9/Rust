
mod math;
fn main() {
    let num1:u32=5;
    let num2:u32=6;
    println!(" {} + {} = {}",num1,num2,math::basic::add(num1, num2));
    println!(" {} + {} = {}",num1,num2,math::advance::multiply(num1, num2));
}
