pub fn main(){
    let vectors = vec![20,20,40,60,70];

    for rs in &vectors{
        match rs{
            60 => println!("second highest number"),
            _ => println!("{:?}",rs)
        }
    }
    println!("Here is the length of the array : {}", vectors.len());
}