fn main(){
    // let nums=[1,2,3,4,5];
    // let mut nums_itr=nums.iter();
    // for n in nums_itr{
    //     println!("{}",n);
    // }
    //println!("{:?}",nums_itr);
    //fetch the values using rust
    // println!("{:?}",nums_itr.next());
    // println!("{:?}",nums_itr.next());
    // println!("{:?}",nums_itr.next());
    // println!("{:?}",nums_itr.next());
    // println!("{:?}",nums_itr.next());

    //types of creating iterators in rust
    //1.Using iter() method
    let colors=vec!["red","blue","pink"];
    let mut colorss=vec!["red","blue","pink"];

    for color in colors.iter(){
        println!("{}",color);
    }

    //2.using into_iter() method
    for i in colors.into_iter(){
        println!("{}",i);
    }

    //here when we print the color vector it will raise error becuse after the iter loop the variable became unavailable

    //3.using iter_mut() method
    for j in colorss.iter_mut(){
        *j="pink";
        println!("{}",j);
    }
}

