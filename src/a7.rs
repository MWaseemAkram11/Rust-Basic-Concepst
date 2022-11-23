fn print_it(data:&str){
    println!("Here is result: {}",data);
}

pub fn main(){
    let owned_string = "owned string".to_owned();
    let another_owned_string = String::from("another owned string");
    print_it(&owned_string);
    print_it(&another_owned_string);
}