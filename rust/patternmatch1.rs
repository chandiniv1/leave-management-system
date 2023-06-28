fn main(){
    let five=Some(5);
    let six=plus_one(five);
    let none=plus_one(None);
    println!("{:?}",six);
    println!("{:?}",none);
    let some=Some(3);
    match some{
        Some(3)=>println!("three"),
        _=>(),
    }

    if let Some(3)= some{
        println!("three");
    }
}

fn plus_one(x:Option<i32>) -> Option<i32>{
    match x{
        None=> None,
        Some(i)=> Some(i+1),
    }
}