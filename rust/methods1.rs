struct Rect{
    l:i8,
    b:i8,
}

impl Rect{
    fn area(&self)->i8{
        self.l*self.b
    }
    fn can_hold(&self,other:&Rect)->bool{
        self.l>other.l && self.b>other.b
    }
}

impl Rect{
    fn square(size:i8)->Self{
        Self{
            l:size,
            b:size,
        }
    }
}
fn main(){
    let rect1=Rect{
        l:12,
        b:14,
    };
    let rect2=Rect{
        l:3,
        b:5,
    };
    let _ rect3=Rect{
        l:6,
        b:7,
    };
    let squares=Rect::square(4);
    println!("area={}",rect1.area());
    println!("areas={}",squares.area());

    println!("can 1 hold 2={}",rect1.can_hold(&rect2));
}