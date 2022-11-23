struct Country{
    c_name:String,c_population:i64
}

fn print_result(data:&str){
    println!("Here is output of the countries list: {:?}", data)
}

pub fn main(){
    let c_vector = vec![
        Country{
            c_name:"KSA".to_owned(),c_population:8
        },
        Country{
            c_name:"Pakistan".to_string(),c_population:25
        },
        Country{
            c_name:"Qatar".to_uppercase(),c_population:15
        },

    ];

    for res in c_vector{
        let country_name = res.c_name;
        print_result(&country_name);
        println!("here is population is : {}", res.c_population);
    }
}