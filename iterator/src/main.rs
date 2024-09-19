use std::vec;

fn main() {
    let mut v=vec![1,2,3];
    let v_itr=v.iter();

    for val in &v {
        println!("{}",val);
    }

    for val in v_itr {
        println!("{}",val);
    }

    let mut v_mutitr=v.iter_mut();
    let first_number=v_mutitr.next();

    match first_number {
        Some(val)=>
            println!("{} is Val",val),
        None => 
            println!("No Value found")
    }

    
    // while let Some(val) = v_mutitr.next(){
    //     println!("{}",val);
    // }





}
