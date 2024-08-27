fn main() {
  
    let mut number: Option<i32> = Some(5);
  
    println!("{:?}", number);
    
    // getting mutable reference
    if let Some(val) = number.as_mut() {
        *val += 7; // updating value inside the option
    }
  
  
      println!("after mutating the value in the option {:?}", number);
  
      let new_value = match number.take() {
      None => None , 
      Some(val) => Some(val +1)   
      };
  
     println!("{:?} new value", new_value);
     println!("{:?} new value", number);
  
  
  
  }
  