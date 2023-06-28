fn main(){
    enum Spread{
        Int(i32),
        Float(f32),
        Text(String),
    }

    let row:Vec<Spread>=vec![
        Spread::Int(3),
        Spread::Float(3.5),
        Spread::Text(String::from("chandu")),
    ];

    match &row[1]{
        Spread::Int(i)=>println!("int{}",i),
        Spread::Float(i)=>println!("flaoat{}",i),
        _=>println!("not int")
    }
}