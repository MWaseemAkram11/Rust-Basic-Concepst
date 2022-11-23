pub fn call_sub_fn(x:i32,y:i32) ->i32{
    x-y
}
pub fn display_result(result:i32){
    println!("here is argumented output is : {}",result);
}

pub fn main(){
    let result = call_sub_fn(5,5);
    display_result(result);
}
