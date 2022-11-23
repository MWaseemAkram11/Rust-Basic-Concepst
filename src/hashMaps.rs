use std::collections::HashMap;
#[derive(Debug)]
struct hash_map{
    content:String,
}

pub fn main(){
    let mut hmp = HashMap::new();
    hmp.insert(
        1, hash_map{
        content:"Hi".to_owned()
    });
    hmp.insert(
        2, hash_map{
        content:"Welcome".to_owned()
    });
    hmp.insert(
        3, hash_map{
        content:"Great".to_owned()
    });
    hmp.insert(
        4, hash_map{
        content:"Stunning".to_owned()
    });

    for(key,content) in hmp.iter(){
        println!("Here is key: {:?} , Here is vaue: {:?}", key,content);
    }
}
