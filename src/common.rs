use std::sync::{Arc,Mutex};
use sea_orm::DatabaseConnection;
use std::convert::From;
use std::ops::Mul;
#[derive(Clone)]
pub struct AppData{
    pub counter: Arc<Mutex<i32>>,
    pub db: DatabaseConnection,
}



#[derive(Debug)]
struct Number{
    value: i32
}


impl From<i32> for Number{
    fn from(value: i32) -> Self {
        Number { value}        
    }
}

impl Mul for Number {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Number{
           value: self.value * other.value
        }
    }
}


#[cfg(test)]


mod tests{

    use super::*;

    #[test]
    fn test_convert() {

            let num =32i32;
            let number = Number::from(num);
            println!("Number is {}", number.value);
            // let nb = Number::from(20);

            assert_eq!(32,number.value);
    }

    #[test]
    fn test_mul_alis(){
        let num =32i32;
        let number = Number::from(num);
        let nb = Number::from(20);
        let c =nb *number;
        assert_eq!(640,c.value);
    }


    #[test]
    fn test_arc_lock(){
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
       
  
       for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);

     
            let handle =std::thread::spawn(move || {
                let mut count = counter_clone.lock().unwrap();
                *count += 1;
                println!("Thread incremented count to {}", *count);
            });

            handles.push(handle);
        }

        // Wait for all threads to finish
        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(10, *counter.lock().unwrap());
    }
}