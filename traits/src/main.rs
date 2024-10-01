
pub trait PrintData{
    fn default(&self){
        println!("Not overidded");
    }
    fn get_data(&self)->String;
}
struct Emp{
    name:String,
    age:u32,
    id:i32
}

impl PrintData for Emp{
    fn get_data(&self) ->String{
        return format!("Employee Name : {} , Age : {} , id : {}",self.name,self.age,self.id);
    }
}
fn main() {
    let emp1=Emp{
        name:String::from("rishi"),
        age:19,
        id:324
    };
    println!(" Employee Data : {} ",emp1.get_data());
    println!(" Employee Age : {} ",emp1.name);
    println!(" Employee Name : {} ",emp1.age);

    notify(&emp1);
}
fn notify(t:&impl PrintData){
    t.default();
}