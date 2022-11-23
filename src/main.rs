use std::hash;

mod a1;
mod a2;
mod a3;
mod a4;
mod a5;
mod a6;
mod vectors;
mod vectors2;
mod a7;
mod a8;
mod options;
mod hashMaps;
mod userinput;
 fn main() {
    let a = "abcd";
    // let b = vec![a];
    println!("a is : {:?}", &a[0..1]);
    for c in a.chars(){
        println!("string iterate or not: {}", c);
    }
    println!("Hello, world!");
    a1::main();
    a2::main();
    a3::main();
    a4::main();
    a5::main();
    a6::main();
    vectors::main();
    vectors2::main();
    a7::main();
    a8::main();
    options::main();
    hashMaps::main();
    userinput::main();
}
