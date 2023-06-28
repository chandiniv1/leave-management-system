//this is the first program in rust
fn main(){
    let x="apple";
    let y=1;
    //let z=2; //unused values will t hrow error
    println!("{}",x);
    println!("y={}",y);
    println!("hello world");
    print!("1");
    print!("{}",2);
    println!();
    println!("x={},y={}",x,y);
    println!("x={x},y={y}");

    //mutability in rust
    let mut p=1;
    println!("{}",p);
    p=2;
    println!("{}",p);

    //empty var
    /*let q;        results an error
    println!("empty var={}",q);  */
}

