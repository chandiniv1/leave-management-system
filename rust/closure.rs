fn main(){
    //defining a closure
    let clos=|| println!("this is a closure");

    //calling the closure
    clos();

    //we can also pass parameters to the closure
    let add=|x:i8| x+1;
    let res=add(1);
    println!("res={}",res);


    //multi-line closure in rust
    let sq_sum=|x:i8,y:i8|{
        let sum:i8=x*x + y*y;
        return sum;
    };
    println!("result={}",sq_sum(5,3));

    //closure environment capturing :means using the variable which is not defined in it
    let a=10;
    let closr=|| println!("num={}",a);
    closr();

    //it has three modes
    //1.variable is not modified inside closure
    
    let word="HIII";
    let word_clos=||{
        println!("word={}",word);
    };
    println!("lenth={}",word.len());
    word_clos();

    //2.variable is modified inside the closure
    let mut s=String::from("hello");
    let mut concat=||{
        s.push_str("world");
        println!("word={}",s)
    };
    concat();

    //3.variable is moved inside closure

    let words="hi";
    let wod=||{
        let new_word=words;
        println!("word={}",new_word);
    };
    wod();

}