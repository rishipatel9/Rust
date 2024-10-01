struct Emp<'a>{
    nane:&'astr
}
fn main() {
    let ans;
    let a=String::from("rishi");{

        let b=String::from("krishna");
        ans=get_longer(&a,&b);
    }
    println!("{}",ans);

    let emp1=Emp{
        name:&a
    }
    println!("{} ",emp1.name);

}
fn get_longer<'a>(a:&'a str,b:&'a str)->&'a str{
    if a.len() > b.len() {a}
    else {b}
} 