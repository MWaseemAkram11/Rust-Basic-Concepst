struct Test{
    score:i32
}

pub fn main(){
    let my_vector_array= vec![
        Test{score:31},
        Test{score:32},
        Test{score:33}, 
        Test{score:34},
        Test{score:35}
    ];
    for res in my_vector_array{
        println!("Here the output in loop: {}", res.score);
    }
}