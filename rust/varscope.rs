fn main(){
    let out=100;
    {
        let inr=200;
        println!("out={}",out);
        println!("in={}",inr);
    }
    println!("out1={}",out);
   // println!("in1={}",inr); cant access the local variable


   //variable shadowing

   let outer=100;
   {
        println!("out={}",outer);
        let outer=200;
        println!("out={}",outer);
   }
   println!("outer={}",outer);

   //variable freezing

   let mut age=100;
   {
       let age=age;
       println!("the inner age={}",age);
   }
   age=3;
   println!("the outer age={}",age)
}