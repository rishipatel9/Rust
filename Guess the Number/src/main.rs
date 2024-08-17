use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    let random_number:u32=rand::thread_rng().gen_range(1..=100);
    loop{
        let mut guess=String::new(); 
        println!("Enter your Guess");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess:u32=match guess.trim().parse() {
            Ok(num)=>num,
            Err(_) => continue
        };
        match guess.cmp(&random_number) {
            Ordering::Less =>println!("Too Small"),
            Ordering::Greater =>println!("Too Big"),
            Ordering::Equal =>{
                println!("You Win!!");
                break;
            }
        }
    }
}
