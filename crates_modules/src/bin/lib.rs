mod test_mod{
    pub fn get(str:String){
        println!("Received string {}",str);
    }
    pub fn increase(num:u32)->u32{
        num +1
    }
}

mod choose_dinner{
    // marking the struct public does not make its members public 
    #[derive(Debug)]
    pub struct Dinner {
        pub sos:String,
        bread:bool
    }
    impl Dinner{
        pub fn choose(opt:&str)->Dinner{
            return Dinner{
                sos:String::from(opt),
                bread:true
            }
        }
    }
}

mod john{
    mod deo{
        pub fn one(){
            println!("one")
        }
    } 
}
fn main(){
    let s:String=String::from("rishi");
    test_mod::get(s);
    let num:u32=5;
    println!("num after increase fn {}",test_mod::increase(num));

    let dinner_meal=choose_dinner::Dinner::choose("tomato ketchup");
    println!("Y  our dinner {:?}",dinner_meal);
}