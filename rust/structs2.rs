#[derive(Debug)]
struct Rect{
    l:i8,
    b:i8,
}

fn area(rectangle:&Rect)->i8{
    rectangle.l*rectangle.b
}

fn main(){
    let rect1=Rect{
        l:2,
        b:4,
    };
    println!("rect={:?}",rect1);
    println!("rect={:#?}",rect1);


    println!("area={}",area(&rect1));
}