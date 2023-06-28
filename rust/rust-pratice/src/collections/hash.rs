use std::collections::HashMap;
fn main(){
    let blue=String::from("blue");
    let pink=String::from("pink");

    let mut scores:HashMap<String,i32>=HashMap::new();
    scores.insert(blue,10);
    scores.insert(pink,10);
    
    let team=String::from("blue");
    let score:Option<&i32>=scores.get(&team);

    //let score:Option<i32>=scores.get(&team).copied();
 
    for (k,v) in &scores{
        println!("{}:{}",k,v);
    }

    let mut scr:HashMap<String,i32>=HashMap::new();
    scr.insert(String::from("blue"),30);
    scr.insert(String::from("blue"),40);//replaces the key
    println!("hash:{:?}",scr);
    scr.entry(String::from("pink")).or_insert(50);
    scr.entry(String::from("pink")).or_insert(60); //doesnot replaces the key
    println!("hash:{:?}",scr);


}