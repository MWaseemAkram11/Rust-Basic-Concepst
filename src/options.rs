struct Demo{
    q1:Option<i32>,
    q2:Option<String>,
    q3:Option<bool>,
}

pub fn main(){
    let result = Demo {
        q1:None,
        q2:Some("oprion 2 string".to_owned()),
        q3:Some(true),
    };

    match result.q1{
        Some(ans) => println!("here us output of q1: {}",ans),
        None => println!("No response from q1"),
    }
    match result.q2{
        Some(ans) => println!("here us output of q2: {}",ans),
        None => println!("No response from q2"),
    }
    match result.q3{
        Some(ans) => println!("here us output of q3: {}",ans),
        None => println!("No response from q3"),
    }
}