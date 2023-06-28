#[derive(Debug)]
struct Rect{
    w:i8,
    l:i8,

}
impl Rect{
    fn can_hold(&self,other:&Rect)->bool{
        self.w>other.w && self.l>other.l
    }
}

pub struct Guess{
    val:i32,
}

impl Guess{
    pub fn new(val:i32)->Guess{
        if val<1 || val>100{
            panic!("worng");
        }
        Guess { val }
    }
}
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }

    #[test]
    fn it_works()->Result<(),String>{
        if add(2,2)==4{
            Ok(())
        }else{
            Err(String::from("error"))
        }
    }

    use super::*;
    #[test]
    fn larger_can_hold(){
        let larger=Rect{
            w:8,
            l:7,
        };
        let smaller=Rect{
            w:5,
            l:1,
        };

        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[should_panic]
    fn greater(){
        Guess::new(200);
    }
}
