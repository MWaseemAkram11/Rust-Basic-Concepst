pub fn call_div_fn(m:u32,n:u32) ->u32{
    m/n
}
pub fn display_result(result:u32){
    println!("Here is result of argumented values:::: {}",result);
}

pub fn main(){
    let result = call_div_fn(10,10);
    display_result(result);
}