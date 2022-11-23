
enum Direction{
    left,
    right
}

pub fn main(){
    let go = Direction::left;
    match go{
        Direction::left => println!("go to left"),
        Direction::right => println!("got right")
    }
}