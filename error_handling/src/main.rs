use std::fs;
fn main() {
    println!("Hello, world!");
    let res=fs::read_to_string("example.txt");

    match  res {
        Ok(content)=>{
            print!("File contains : {}",content);
        },
        Err(err)=>{
            print!("Error :{}",err);
        }
    }
    print!("Operation completed");
}
