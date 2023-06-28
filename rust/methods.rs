#[derive(Debug)]
struct Rect{
    len:i8,
    br:i8,
}

impl Rect{
    fn area(&self)->i8{
        self.len*self.br
    }
    fn width(&self)->bool{
        self.br>1
    }
}

fn main(){
    let rect1=Rect{
        len:4,
        br:3,
    };
    if rect1.width(){
        println!("width");
    }
    println!("area={}",rect1.area());

}