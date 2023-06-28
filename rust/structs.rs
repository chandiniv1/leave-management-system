#[derive(Clone)]

struct User{
    active:bool,
    name:String,
    email:String,
    sign_in:u8,
}

struct Test<'a>{
    name:&'a str,
    id:i8,
}

fn build_user(email:String,u_name:String)->User{
    User{
        active:true,
        name:u_name,
        email:email,
        sign_in:1,
    }
}

//tuple structs without named fields
struct Color(i32,i32,i32);

//unit-like strucct
//struct Always;
fn main(){
    let black=Color(1,2,3);
    println!("{}",black.0);
    // let always_equal=Always;
    // println!("{}",always_equal.0);
    let mut user1=User{
        active:true,
        name:String::from("chandini"),
        email:String::from("chandini@gmail.com"),
        sign_in:3,
    };

    let user3=User{
        active:true,
        name:user1.name.clone(),
        email:String::from("dsfnlferfre"),
        sign_in:4,
    };
    user1.active=false;
    println!("{}",user1.active);
    println!("{}",user3.name);
    let user2=build_user(String::from("qwer@gmail.com"),String::from("chandu"));
    println!("{}",user2.name);

    let user4=User{
        active:true,
        ..user1
    };
    println!("user4:{}",user4.name);

    let test1=Test{
        name:"qwerty",
        id:2,
    };
    println!("{}",test1.name);
}