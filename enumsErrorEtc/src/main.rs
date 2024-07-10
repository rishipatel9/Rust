
enum Shape{
    Circle(f64),
    Square(f64),
    Rectange(f64,f64)
}

fn calculate_area(shape:Shape)->f64{
    let ans=match shape{
        Shape::Circle(radius)=>3.14 *radius*radius,
        Shape::Square(side)=> side*side,
        Shape::Rectange(width,length) => width*length
    };
    return ans;
}
fn main() {
    //let my_Dir:Direction=Direction::North;
    //let ans=getPos(my_Dir:Direction);
    let circle:Shape=Shape::Circle(3.0);
    let area=calculate_area(circle);
    let square_area=calculate_area(Shape::Square(7.3));
    let rectange_area=calculate_area(Shape::Rectange(10.3,10.6));
    println!("circle area {} , Square Area :{} ,RectangeArea: {}",area,square_area,rectange_area);
    println!("Hello, world!");
}
