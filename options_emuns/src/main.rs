fn find_first_a(s:String) -> Option<usize>{
    for(index,char) in s.chars().enumerate(){
        if char== 'a'{
            return Some(index);
        }
    }
    return None;
}
fn main() {
        
    let s=String::from("harkirat");
    let res=find_first_a(s);

    match res{
        Some(index) =>{
            print!("The first index of a is {}",index);
        },
        None=>{
            print!("There was no a ");
        }
    }
}
