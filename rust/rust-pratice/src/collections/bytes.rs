fn main(){
    let mut s=String::from("chandu");
    s.push_str("hello");
    s.push('!'); //used to push char
    println!("{}",s);

    let s1=String::from("chandu");
    let s2=String::from("hello");
    let s3=s1+&s2;
    println!("{}",s3);

   // let c=s[0]; string cannot be indexed by integer
   for b in s.bytes(){
    //println!("{}",b.to_string());
    println!("{}",b);
   }

   for b in s.chars(){
    println!("{}",b);
   }

}