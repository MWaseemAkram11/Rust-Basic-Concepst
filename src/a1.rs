
pub fn call_add_fn(a:i32,b:i32) -> i32{
    a+b
}

pub fn display_result(result:i32){
    println!("print argumented result is: {}",result);
}
pub fn main(){
    let result = call_add_fn(5, 5);
    display_result(result);
}
