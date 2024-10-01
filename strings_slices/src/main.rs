fn main() { 
    let str=String::from("rishi patel");
    let ans1=get_first_slice(&str);
    println!("first word {}",ans1);

    // let ans=get_first_word(&str);
    // println!("first word {}",ans);

}
// without slice
fn get_first_word(str:&String)->String{
    let mut ans=String::from("");
    for i in str.chars(){
        println!("char {}",i);
        if i == ' '{
            break;
        }
        ans.push_str(&i.to_string());
    }
    ans
}

fn get_first_slice(str:&String) ->&str{
    let mut idx=0;
    for i in str.chars(){
        if i == ' '{
            break;
        }
        idx=idx+1;
    }
    &str[0..idx]
}
