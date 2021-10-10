
fn fact(n: i32) -> i32 {
    if n == 0 {
        return 1;
    } 
        return fact(n-1)*n;
}


fn fact_with_loops(n: i32)-> i32{
    let mut i: i32 =1;
    let mut f: i32 = 1;
    while i <= n {
        f = f * i;
        i = i + 1;
    }
    return f;

}


fn main(){
    let result = fact_with_loops(5);
    println!("{}", result)
}