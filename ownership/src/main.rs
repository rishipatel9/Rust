
struct User{
    name:String,
    active:bool,
    age:u32,
}
fn main() {
    // let mut s:String =String::from("rishi");
    // let  s2: &mut String =&mut s;
    // s2.push_str("patel");
    // println!("{}",s);

    let u1:User=User{
        name:String::from("rishi"),
        active:true,
        age:19
    };
    println!("my name is {} and my age is {}",u1.name,u1.age);
}
