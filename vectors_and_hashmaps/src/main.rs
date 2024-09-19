use std::collections::HashMap;

fn main() {
    let mut v1:Vec<i32>=Vec::new();
    v1.push(1);
    v1.push(3);
    v1.push(4);
    println!("{:?}",v1);
    let v2=vec![1,2,3];
    println!("{:?}",v2);

    let third:&i32=&v1[2];
    println!("The third element of v1 is {}",third);
    let mut idx:usize=0;
    loop{
        match v1.get(idx) {
            Some(&value)=>{
                println!("Value : {} Idx : {}",value,idx);
                idx+=1;
            },
            None=> break,            
        }
    }
    println!("Accessing 100th idx of v1 via &v1.get(100) : {:?}  ",&v1.get(100));
    // println!("Accessing 100th idx of v1 via &v1[100] {} ",&v1[100]);
    for i in &v1{
        println!("{}",i);
    }
    #[derive(Debug)]
    enum Choice{
        One(i32),
        Two(String),
        Three(f32)
    }

    let mut v3=vec![
        Choice::One(6),
        Choice::Two("hello".to_string()),
        Choice::Three(2.3)
    ];
    v3.push(Choice::Two("world".to_string()));

    for i in &v3{
        println!("{:?}",i);
    }

    let mut users:HashMap<String,i32>=HashMap::new();
    users.insert(String::from("rishi"),19);
    users.insert(String::from("dishit"),21);
    let rishi_age=users.get("rishi");
    match rishi_age {
        Some(age) =>
            println!("{:?}",age),
        None => 
            print!("User not found")
    }

    let v=vec![(String::from("rishi"),19),(String::from("krishna"),17)];
    let mp=group_values(v);
    print!("{:?}",mp);




}

fn group_values(vec:Vec<(String,i32)>)->HashMap<String,i32>{
    let mut ans=HashMap::new();
    for(key,value) in vec{
        ans.insert(key,value);
    }
    return ans;
}

// fn filter(vec:&Vec<i32>)->Vec<i32>{
//     let mut ans=Vec::new();
//     for val in vec{
//         if val%2==0{
//             ans.push(*val);
//         }
//     }
//     return ans;
// }