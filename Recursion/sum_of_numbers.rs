

fn sum(number: i32) -> i32{
    if number == 0{
        return 0;
    }
    else{
        return sum(number-1)+number;
    }
}


fn main(){
    let result = sum(5);
    println!("{}", result)    
}