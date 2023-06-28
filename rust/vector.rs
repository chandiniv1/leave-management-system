fn main(){
    let v=vec![1,2,3,4];
    let vec:Vec<u8>=vec![1,2,3,5,6];
    println!("v ={:?}",v);
    println!("vec={:?}",vec);
    println!("v at 0={:?}",v.get(0));

    //adding values to the vector
    let mut ve=vec![5,4,3,2,1];
    ve.push(6);
    ve.push(7);
    println!("changed vec={:?}",ve);

    //removing values to the  vector based on the index
    ve.remove(1);
    println!("modified vec={:?}",ve);

    //creating a vector using vec::new() method
    let mut vect:Vec<i32>=Vec::new();
    vect.push(0);
    vect.push(1);
    println!("vector={:?}",vect);

}