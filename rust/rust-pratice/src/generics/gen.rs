fn get_largest<T:PartialOrd+Copy>(list:Vec<T>)->T{
    let mut largest=list[0];
    for n in list{
        if n>largest{
            largest=n;
        }
    } 
    largest
}

fn main(){
    let list:Vec<i32>=vec![10,20,30,5,2];
    let largest=get_largest(list);
    println!("large={}",largest);

    let list2:Vec<char>=vec!['a','b','c','d'];
    let largest=get_largest(list2);
}