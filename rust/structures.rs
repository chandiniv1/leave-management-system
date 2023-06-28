fn main(){
    struct Person{
        name:String,
        age:i8,
        height:u32
    }

    let p1=Person{
        name:String::from("chandu"),
        age:18,
        height:164
    };
    
    println!("person name={}",p1.name);
    println!("person age={}",p1.age);
    println!("person height={}",p1.height);
   // println!("person={:?}",&p1); resulting an error

   //destructing the struct 
    let Person {name,age,height}=p1;
    println!("person name={}",name);
    println!("person age={}",age);
    println!("person height={}",height);
}