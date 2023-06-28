fn main(){
    let a:[i32;3]=[1,2,3];
    let mut v:Vec<i32>=Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v1={:?}",v);

    let mut v2:Vec<i32>=vec![1,2,3];
    v2.push(4);
    println!("v2={:?}",v2);

    let third=&v[2]; 
    //v.push(5); error because of immutable borrow
    println!("{}",third);

    match v.get(2){
        Some(third)=>println!("{}",third),
        None=>println!("none"),
    }

    //iterating over  vector

    for mut i in &mut v{
        *i+=1;
        println!("{}",i);
    }

}