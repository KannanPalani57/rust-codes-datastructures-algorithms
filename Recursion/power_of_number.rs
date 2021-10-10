
fn power(m: i32, n: i32)-> i32{
    if n==0 {
        return 1;
    }
    else {
        return power(m, n-1) * m;
    }
}

fn another_way_power(m: i32, n: i32) -> i32{
    if n== 0{
        return 1;
    }
    else if n%2 == 0 {
        return another_way_power(m*n, n/2);
    }

    return m * another_way_power(m*m, (n-1)/2);
}

fn main(){
  let result = another_way_power(2,3);
  println!("{}", result);
}