use std::collections::HashMap;

fn main(){
    let mut info:HashMap<i32,String>=HashMap::new();

    //adding
    info.insert(1,String::from("Apple"));
    info.insert(2,String::from("mango"));
    info.insert(3,String::from("guava"));
    println!("map={:?}",info);

    //access values
    let fruits=info.get(&1);
    println!("map={:?}",fruits);

    //remove values
    info.remove(&2);
    println!("map={:?}",info);

    //change elements of hashmap
    info.insert(1,String::from("banana"));
    println!("map={:?}",info);
      
}