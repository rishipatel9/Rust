

fn main() {
    let i: i32 = 1234567123;
    let x: i64 = -2345;
    // let y:i8=127;

    let name: String = String::from("rishi");

    println!("name:{}", name);

    println!("i:{}", i);
    println!("x:{}", x);

    getchars(name);

    println!("2+5 {}", sum(2, 5));

    let mut s=String::from("rishi");
    let _s2= &mut s;
    borrow(&mut s);
    print!("{}",s);
}
fn borrow(str:&mut String){
    str.push_str("patel");
}
fn getchars(name: String) -> String {
    let mut ans = String::from("");
    for char in name.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;
}
fn sum(num1: i32, num2: i32) -> i32 {
    let ans: i32 = num1 + num2;
    return ans;
}
