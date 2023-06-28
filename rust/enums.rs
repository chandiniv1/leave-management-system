#[derive(Debug)]
enum IpAddrType{
    V4(String),
    V6(u8,u8,u8,u8),
}
struct IpAddr{
    kind:IpAddrType,
    address:String,
}



fn main(){
    let home=IpAddr{
        kind:IpAddrType::V4(String::from("dfd")),
        address:String::from("444234234"),
    };
    let ofc=IpAddr{
        kind:IpAddrType::V6(1,2,3,4),
        address:String::from("444234234"),
    };
    println!("kind={:?}",ofc.kind);
    println!("kind={:?}",home.kind);
}