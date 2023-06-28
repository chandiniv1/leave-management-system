use std::collections::HashMap;
fn main(){
    let text="hello world hello";
    let mut freq:HashMap<String,i32>=HashMap::new();

    for word in text.split_whitespace(){
        let mut count=freq.entry(word.to_string()).or_insert(0);
        *count+=1;
    }
    println!("{:?}",freq);
} 