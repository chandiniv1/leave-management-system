#[derive(Debug)]
enum UsState{
    A,
    B,
    C,
    D,
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value(coin:Coin)->u8{
    match coin{
        Coin::Penny=> {
            println!("penny");
            1
        }
        Coin::Nickel=>4,
        Coin::Dime=>5,
        Coin::Quarter(state)=> {
            println!("state{:?}",state);
            25
        }
    }
}

fn main(){
    value(Coin::Quarter(UsState::A));
}