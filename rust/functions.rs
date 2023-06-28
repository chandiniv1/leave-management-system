fn greet(){
    println!("hello world");
}

fn add(a:i32,b:i32){
    let sum=a+b;
    println!("sum={}",sum);
}

fn sub(a:i32,b:i32)->i32{
    let sum=a+b;
    return sum;
}

fn area(dimensions:(i8,i8))->i8{
    dimensions.0*dimensions.1
}

fn main(){
    greet();
    add(10,20);
    let rect=(10,20);
    println!("area={}",area(rect));
    println!("sum={}",sub(20,30));
}

