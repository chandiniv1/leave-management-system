fn main(){
    let mut word=String::from("Hello ");
    word.push_str("world");
    let slice=&word[0..3];
    println!("word={}",word);
    println!("slice={}",slice);

    // for char in word.chars(){
    //     println!("{}",char);
    // }

    for char in word.chars(){
        println!("{}",char);
    }

    let mut word1=String::new();
    word.push_str("hello world");
    println!("changed str={}",word1);
}