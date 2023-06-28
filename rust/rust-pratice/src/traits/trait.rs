
#[derive(Debug)]
pub struct Article{
    pub head:String,
    pub loc:String,
    pub author:String,
    pub content:String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Article{
    fn query(&self)->String{
        format!("REad")
    }
    fn summarize(&self)->String{
        format!("{},by {} ({})",self.head,self.loc,self.author)
    }
}

impl Summary for Tweet{
    fn query(&self)->String{
        format!("REad.....")
    }
    // fn summarize(&self)->String{
    //     format!("{},by {}",self.username,self.content)
    // }
}

pub trait Summary{
    fn query(&self)->String;
    fn summarize(&self)->String{
        String::from("Read more....")
    }
}

fn return_summarize()->impl Summary{
    Tweet{
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
// pub fn notify(item:&impl Summary){
//     println!("Breaking :{}",item.summarize());
// }

// pub fn notify(item1:&(impl Summary+Display),item2:&impl Summary){
//     println!("Breaking :{}",item.summarize());
// }
use std::fmt::Debug;
use std::fmt::Display;
pub fn notify<T:Summary+Display>(item1:&T,item2:&T){
    println!("Breaking :{}",item1.summarize());
}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    35
}


fn main(){
    let tweet=Tweet{
        username:String::from("mary"),
        content:String::from("babby"),
        reply:false,
        retweet:false,
    };

    let article=Article{
        head:String::from("john"),
        loc:String::from("hello"),
        author:String::from("world"),
        content:String::from("Doe"),
    };

    println!("Atricle{}",article.summarize());
    println!("tweet{}",tweet.summarize());
    notify(&article);
    println!("return_summarizable={:?}",return_summarize());
}