
// Struct QuitMsg;
// Struct MoveMsg{
//     x:i32,
//     y:i32,
// }
//struct WriteMsg(String);
//struct ChangeColor(i32,i32,i32);

fn main(){
    enum Message{
        Quit,
        Move{x:i32,y:i32},
        Write(String),
        ChangeColor(i32,i32,i32),
    }

    impl Message{
        fn call(&self)->String{
            String::from("Yes")
        }
    }
    let m=Message::Write(String::from("asfdf000"));
    println!("{}",m.call());

    let some_number:Option<i32>=Some(5);
    let some_string:Option<&str>=Some("a string");
    println!("some_num {:?}",some_number);
    println!("some_str {:?}",some_string);
    let x:i8=10;
    let y:Option<i8>=Some(20);
    let z:Option<i8>=None;
    let sum=x+y.unwrap_or(0);
    let sum1=x+z.unwrap_or(0);
    println!("sum={}",sum);
    println!("sum={}",sum1);
    
}