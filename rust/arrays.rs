fn main(){
    //array without data type
    let arr=[1,2,3,4,5];
    println!("arr={:?}",arr);
    for i in arr{
        println!("{}",i);
    }
    println!("length of array ={}",arr.len());
    for i in 0..arr.len(){
        println!("{}",arr[i]);
    }

    //array with data type
    let a:f32=1.1;
    println!("{}",a);

    let arr1:[i32;5]=[1,1,1,1,1];
    println!("arr1={:?}",arr1);

    let arr2:[i32;5]=[3;5];
    println!("arr2={:?}",arr2);

    let mut arr3=[4;5];
    println!("arr3={:?}",arr3);
    arr3[3]=5;
    println!("arr3={:?}",arr3);
}