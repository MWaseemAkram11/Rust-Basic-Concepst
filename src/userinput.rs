use std::io;

fn get_string() -> io::Result<String> {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_uppercase())
}

pub fn main(){

    let mut counter = 0;
    let mut vector_arr = vec![];
    while(counter < 2){
        match get_string() {
            Ok(word) => {
                vector_arr.push(word);
                counter += 1;
            }
            Err(e) => println!("Something goes wrong!!")
        }
    }

    for res in vector_arr{
        println!("origional : {:?}, after capatilized: {:?}", res, res.to_uppercase());
    }
}