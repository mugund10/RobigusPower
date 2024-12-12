fn main()
{   // it is not mutable
    let n :i32 = 5;
    // this will no mutable
    let mut m:i32 = 5;
    m += 5;
    println!("non mutable value = {n}, and mutable value {}",m);
}
