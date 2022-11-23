pub fn call_mul_fn(j:i32,k:i32) ->i32{
    j*k
}

pub fn display_result(result:i32){
    println!("Here is result of argumented values:::: {}",result);
}

pub fn main(){
    let result = call_mul_fn(10,10);
    display_result(result);
}