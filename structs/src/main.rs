struct User{
    id:u64,
    name:String,
    last_name:String,
    age:u64,
    active:bool,
}

#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32
}
impl Rectangle {
    fn area(&self)->u32{
        return self.width * self.height;
    }
    fn get_width(&self)->u32{
        return self.width;
    }
    fn get_height(&self)->u32{
        return self.height;
    }
    fn can_hold(&self,other:&Rectangle)->bool{
        return self.width > other.width && self.height > other.height
    }
}
fn main() {
    // let user1= User{
    //     id:1,
    //     name:String::from("Rishi"),
    //     last_name:String::from("Patel"),
    //     active:true,
    //     age:19
    // };
    // // similar to spread operator in ts
    // let user2=User{
    //     name:String::from("krishna"),
    //     age:17,
    //     ..user1
    // };
    // print!("last_name derived from user1 to user2 {}",user2.last_name);

    let rect1=Rectangle{
        width:20,
        height:15
    };
    println!("Rectangle 1 :{:?}",rect1);
    let rect2:Rectangle=Rectangle { width: dbg!(12*2), height: (24) };
    print!("Rectangle 2 :{:#?} Area of Rectangle 2:{}",rect2,rect2.area());
    print!("Can Rectangle 2 hold the rectangle one {}",rect2.can_hold(&rect1));


}