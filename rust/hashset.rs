use std::collections::HashSet;

fn main(){
    let mut info:HashSet<&str>=HashSet::new();
    info.insert("red");
    info.insert("pink");
    info.insert("blue");
    info.insert("blue");
    if info.contains("red"){
        println!("red is present in the set");
    }
    info.remove("red");
    for i in &info{
        println!("{}",i);
    }
    println!("hash set={:?}",info);

    //creating hashset with different values
    let nums=HashSet::from([1,2,3,4]);
    let nums2=HashSet::from([1,2,5]);
    println!("hash set={:?}",nums);
    println!("hash set={:?}",nums2);


    //set operations
    //union of sets
    let res:HashSet<_>=nums.union(&nums2).collect();
    println!("result={:?}",res);
    
    //intersection of two sets
    let res1:HashSet<_>=nums.intersection(&nums2).collect();
    println!("result={:?}",res1);
    
    //difference of two sets
    let res2:HashSet<_>=nums.difference(&nums2).collect();
    println!("result={:?}",res2);

    //symmetric difference of two sets
    let res3:HashSet<_>=nums.symmetric_difference(&nums2).collect();
    println!("result={:?}",res3);

}