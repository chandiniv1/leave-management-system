// fn main(){
//     a();
// }

// fn a(){
//     b();
// }

// fn b(){
//     c(22)
// }

// fn c(num:i32){
//     if num==22{
//         panic!("paniced");
//     }
// }


// <----------------RESULT ENUM---------------->

// fn main(){
//     enum Result<T,E>{
//         Ok(T),
//         Err(E),
//     }
// }

use std::fs::File;
use std::io::ErrorKind;

fn main(){
    let f=File::open("hello.txt");

    let f:File=match f{
        Ok(fil)=>fil,
        Err(error)=>match error.kind(){
            ErrorKind::NotFound=> match File::create("hello.txt"){
                Ok(fil)=>fil,
                Err(e)=>panic!("problem creating the file"),
            },
            other=>{
                panic!("problem in opening the file{:?}",other)
            }
        }
    };
    println!("{:?}",f);

    //in place of this match we can just usee unwrap

    // let mut s=File::open("a.txt")?;
    // let mut s1=String::new();
    // s.read_to_string(&mut s1)?;
    // Ok(s1)
}

