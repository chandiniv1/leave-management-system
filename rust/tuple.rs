fn main(){
    let tuple=(1,'A',"hello");
    println!("tuple={:?}",tuple);

    let tuple2:(i8,char,&str)=(2,'a',"hii");
    println!("tuple2={:?}",tuple2);

    println!("tuple of 1={}",tuple.1);

    //mutable tuple
    let mut tup=(1.1,2,'s');
    // tup.0="2.5"; //results an error because we can replace with only float
    tup.0=2.5;
    println!("tup={:?}",tup);

    //destructing a tuple
    let person=("chandu",21,164);
    let (name,age,height)=person;
    println!("name={}",name);
    println!("age={}",age);
    println!("height={}",height);
}