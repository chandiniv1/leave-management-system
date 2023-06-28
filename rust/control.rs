fn main(){
    let mut num=5;
    if num>5{
        println!("number is greater than 5");
    }else{
        println!("number is less t han or equal to 5");
    }
    while num<10{
        println!("{} is less than 10",num);
        num+=1;
    }
    for i in 10..20{
        println!("{}",i);
    }

    while num<10{
        break;
    }
    for i in 1..10{
        if i==5{
            continue;
        }else if i==8{
            break;
        }else{
        println!("{}",i);
        }
    }

}