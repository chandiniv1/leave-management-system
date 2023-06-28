#[derive(Debug)]
struct Point<T,U>{
    x:T,
    y:U,
}

impl<T,U> Point<T,U>{
    fn mixup<T1,U1>(self,other:Point<T1,U1>)->Point<T,U1>{
        Point{
            x:self.x,
            y:other.y,
        }
    }
}

fn main(){
    let p=Point{x:10,y:2.0};
    let p2=Point{x:2.3,y:30};
    let p3=Point{x:'c',y:30};
    let p4=p.mixup(p2);
    println!("{:?}",p4);
}