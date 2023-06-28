fn main(){
    let nums=[1,2,3,4,5];
    let slice1=&nums[0..4];
    println!("nums={:?}",nums);
    println!("slice={:?}",slice1);

    let slice2=&nums[..nums.len()];
    println!("slice2={:?}",slice2);

    let slice3=&nums[0..];
    println!("slice3={:?}",slice3);

    let slice4=&nums[..];
    println!("slice4={:?}",slice4);

    let mut nums2=[5,8,3,11,1];
    let slice5=&mut nums2[..];

    slice5[2]=10;
    println!("slice4={:?}",slice5);

}