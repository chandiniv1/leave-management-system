#[derive(Debug)]
struct Point<T,U>{
    x:T,
    y:U,
}

fn main(){
    let p1=Point{x:5,y:10};
    let p2=Point{x:'a',y:10};
    println!("p1={:?}",p1);
    println!("p2={}",p1.x);

    // enum Option<T>{
    //     Some(T),
    //     None,
    // }
    // enum Result<T,E>{
    //     Ok(T),
    //     Err(E),
    // }
}